use std::collections::{BTreeMap};

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, Without},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::Vec2,
    time::{Fixed, Time},
    transform::{
        components::{Transform},
    },
};

use super::{broad_phase, get_collision, Collider, Collision, PhysicsSettings};

pub enum BodyType {
    Dynamic { mass: f32 },
    Fixed,
}

#[derive(Component)]
pub struct RigidBody {
    pub body_type: BodyType,
    pub friction_coef: f32,
}

#[derive(Component)]
pub(super) struct RigidBodyInternal {
    pub mass: f32,
    pub inv_mass: f32,
    pub impulse: Vec2,
    pub pseudo_impulse: Vec2,
    pub friction_coef: f32,
    pub accumulated_impulse: Vec2,
}

impl RigidBodyInternal {
    fn new(rb: &RigidBody) -> Self {
        RigidBodyInternal {
            mass: match rb.body_type {
                BodyType::Dynamic { mass } => mass,
                BodyType::Fixed => 0.0,
            },
            inv_mass: match rb.body_type {
                BodyType::Dynamic { mass } => 1. / mass,
                BodyType::Fixed => 0.0,
            },
            impulse: Vec2::ZERO,
            pseudo_impulse: Vec2::ZERO,
            friction_coef: rb.friction_coef,
            accumulated_impulse: Vec2::ZERO,
        }
    }

    fn get_total_velocity(&self) -> Vec2 {
        (self.impulse + self.accumulated_impulse + self.pseudo_impulse) * self.inv_mass
    }

    fn add_impulse(&mut self, impulse: Vec2) {
        self.accumulated_impulse += impulse;
    }

    fn add_pseudo_impulse(&mut self, impulse: Vec2) {
        self.pseudo_impulse += impulse;
    }

    fn apply_accumulated_impulse(&mut self) {
        self.impulse += self.accumulated_impulse;
        self.accumulated_impulse = Vec2::ZERO;
    }
}

pub(super) struct Contact {
    pub impulse: f32,
    pub collision: Collision,
    pub tick: u32,
}

impl Contact {
    fn new(tick: u32, collision: Collision) -> Contact {
        Contact {
            impulse: 0.0,
            collision,
            tick,
        }
    }

    fn update(&mut self, tick: u32, collision: Collision) {
        self.impulse = self.collision.normal.dot(collision.normal).max(0.0) * self.impulse * 0.99;
        self.collision = collision;
        self.tick = tick;
    }
}

#[derive(Resource, Default)]
pub(super) struct ContactCache {
    contacts: BTreeMap<(Entity, Entity), Contact>,
    current_tick: u32,
}

impl ContactCache {
    fn update_contacts<'a>(&mut self, pairs: impl Iterator<Item = (Entity, Entity, Collision)>) {
        self.current_tick += 1;
        for (a, b, collision) in pairs {
            if let Some(contact) = self.contacts.get_mut(&(a, b)) {
                contact.update(self.current_tick, collision);
            } else if let Some(contact) = self.contacts.get_mut(&(b, a)) {
                contact.update(self.current_tick, collision.reversed());
            } else {
                self.contacts
                    .insert((a, b), Contact::new(self.current_tick, collision));
            }
        }

        self.contacts.retain(|_, c| c.tick == self.current_tick);
    }
}

pub(super) fn physics(
    mut bodies: Query<(
        Entity,
        &Collider,
        &mut Transform,
        Option<&mut RigidBodyInternal>,
    )>,
    time: Res<Time<Fixed>>,
    physics_settings: Res<PhysicsSettings>,
    mut contact_cache: ResMut<ContactCache>,
) {
    let dt = time.delta_seconds();
    let g = physics_settings.gravity;

    // Integration
    for body in bodies.iter_mut() {
        if let (_, _, mut t, Some(mut rb)) = body {
            let delta_t = rb.get_total_velocity() * dt;
            let delta_i = g * rb.mass * dt;

            t.translation += delta_t.extend(0.);
            rb.accumulated_impulse += delta_i;

            rb.pseudo_impulse = Vec2::ZERO;
        }
    }

    let collider_transforms = bodies.iter().map(|(e, c, t, _)| (e, c, t));
    let pairs = broad_phase(collider_transforms);

    let mut collisions = pairs
        .iter()
        .map(|(a, b)| {
            let [ae, be] = bodies.get_many_mut([*a, *b]).unwrap();
            let (_, ac, at, _) = ae;
            let (_, bc, bt, _) = be;
            (*a, *b, get_collision(ac, &at, bc, &bt))
        })
        .filter(|(_, _, c)| c.is_some())
        .map(|(a, b, c)| (a, b, c.unwrap()))
        .collect::<Vec<_>>();

    contact_cache.update_contacts(collisions.drain(..));
    const WARMSTART: bool = true;

    for ((a, b), contact) in contact_cache.contacts.iter_mut() {
        let [ae, be] = bodies.get_many_mut([*a, *b]).unwrap();
        if let (Some(mut ab), Some(mut bb)) = (ae.3, be.3) {
            if ab.inv_mass + bb.inv_mass < 1e-9 {
                continue;
            }

            let friction_impulse = contact.impulse * ab.friction_coef.min(bb.friction_coef);
            let tangent = Vec2::Y.rotate(contact.collision.normal);
            let tangent_velocity = tangent.dot(ab.get_total_velocity() - bb.get_total_velocity());
            let tangent_impulse = tangent_velocity / (ab.inv_mass + bb.inv_mass);
            let impulse = friction_impulse.min(tangent_impulse.abs()) * tangent;
            ab.add_impulse(-impulse * tangent_impulse.signum());
            bb.add_impulse(impulse * tangent_impulse.signum());
        }
    }

    for ((a, b), contact) in contact_cache.contacts.iter_mut() {
        let [ae, be] = bodies.get_many_mut([*a, *b]).unwrap();
        if let (Some(mut ab), Some(mut bb)) = (ae.3, be.3) {
            if WARMSTART {
                ab.add_impulse(-contact.impulse * contact.collision.normal);
                bb.add_impulse(contact.impulse * contact.collision.normal);
            } else {
                contact.impulse = 0.0;
            }
        }
    }

    let mut contacts = contact_cache
        .contacts
        .iter_mut()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();
    contacts.sort_by_key(|a| (-a.1.collision.point.dot(g)) as i64);
    const STEPS: u32 = 16;
    for _ in 0..STEPS {
        for ((a, b), contact) in contacts.iter_mut() {
            let [ae, be] = bodies.get_many_mut([*a, *b]).unwrap();

            if let (Some(mut ab), Some(mut bb)) = (ae.3, be.3) {
                if ab.inv_mass + bb.inv_mass < 1e-9 {
                    continue;
                }
                let av = ab.get_total_velocity();
                let bv = bb.get_total_velocity();
                let v = contact.collision.normal.dot(av - bv);
                let mut impulse = v / (ab.inv_mass + bb.inv_mass);
                if impulse < 0.0 {
                    impulse = impulse.max(-contact.impulse);
                }
                ab.add_impulse(-impulse * contact.collision.normal);
                bb.add_impulse(impulse * contact.collision.normal);
                contact.impulse += impulse;
            }
        }
    }

    for _ in 0..STEPS {
        for ((a, b), contact) in contacts.iter_mut() {
            let [ae, be] = bodies.get_many_mut([*a, *b]).unwrap();

            if let (Some(mut ab), Some(mut bb)) = (ae.3, be.3) {
                if ab.inv_mass + bb.inv_mass < 1e-9 {
                    continue;
                }
                let av = ab.get_total_velocity();
                let bv = bb.get_total_velocity();
                let v = contact.collision.normal.dot(av - bv) + (contact.collision.depth - 1e-3);
                let mut impulse = v / (ab.inv_mass + bb.inv_mass);
                if impulse < 0.0 {
                    impulse = 0.0;
                }
                ab.add_pseudo_impulse(-impulse * contact.collision.normal);
                bb.add_pseudo_impulse(impulse * contact.collision.normal);
            }
        }
    }

    // Apply
    for body in bodies.iter_mut() {
        if let Some(mut rb) = body.3 {
            rb.apply_accumulated_impulse();
        }
    }
}

pub(super) fn update_rigid_body_internal(
    mut commands: Commands,
    without_rbi: Query<(Entity, &RigidBody), Without<RigidBodyInternal>>,
    mut changed_rbi: Query<(&RigidBody, &mut RigidBodyInternal), Changed<RigidBody>>,
) {
    for (e, rb) in without_rbi.iter() {
        commands.entity(e).insert(RigidBodyInternal::new(rb));
    }

    for (rb, mut rbi) in changed_rbi.iter_mut() {
        *rbi = RigidBodyInternal::new(rb);
    }
}
