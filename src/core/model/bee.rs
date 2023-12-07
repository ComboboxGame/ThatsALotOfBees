use super::BeeMaterial;

use bevy::prelude::*;

use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, EnumIter, Component)]
pub enum BeeKind {
    #[default]
    Baby,
    Regular,
    Worker,
    Builder,
    Defender,
    Queen,
}

#[derive(Component)]
pub struct LivingCreature {
    pub time_alive: f32,
    pub health: i32,
    pub damage: u32,
    pub attack_cooldown: f32,
    pub time_since_last_attack: f32,
    pub time_since_last_damage_taken: f32,
    pub push_back: Vec2,
}

impl Default for LivingCreature {
    fn default() -> Self {
        Self {
            time_alive: Default::default(),
            health: Default::default(),
            damage: Default::default(),
            attack_cooldown: 10.0,
            time_since_last_attack: Default::default(),
            time_since_last_damage_taken: 1000.,
            push_back: Vec2::ZERO,
        }
    }
}

impl LivingCreature {
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn can_attack(&self) -> bool {
        self.time_since_last_attack > self.attack_cooldown
    }

    pub fn attack(&mut self, other: &mut LivingCreature, direction: Vec2) {
        if !other.is_dead() && self.damage > 0 && self.can_attack() {
            other.health -= self.damage as i32;
            self.time_since_last_attack = 0.0;
            other.time_since_last_damage_taken = 0.0;
            other.push_back +=
                direction.normalize_or_zero() * (self.damage as f32).powf(0.6).min(3.0);
            self.push_back -=
                direction.normalize_or_zero() * (self.damage as f32).powf(0.4).min(2.0) * 0.8;
        }
    }
}

impl From<BeeKind> for LivingCreature {
    fn from(value: BeeKind) -> Self {
        match value {
            BeeKind::Baby => LivingCreature {
                health: 1,
                damage: 0,
                attack_cooldown: 10.0,
                ..Default::default()
            },
            BeeKind::Regular => LivingCreature {
                health: 2,
                damage: 1,
                attack_cooldown: 1.5,
                ..Default::default()
            },
            BeeKind::Worker => todo!(),
            BeeKind::Builder => todo!(),
            BeeKind::Defender => LivingCreature {
                health: 4,
                damage: 2,
                attack_cooldown: 2.5,
                ..Default::default()
            },
            BeeKind::Queen => LivingCreature {
                health: 60,
                damage: 2,
                attack_cooldown: 2.0,
                ..Default::default()
            },
        }
    }
}

pub fn update_bee_material_system(
    mut commands: Commands,
    mut bees: Query<(Entity, &BeeKind), Changed<BeeKind>>,
    mut materials: ResMut<Assets<BeeMaterial>>,
) {
    for (e, bee) in bees.iter_mut() {
        commands.entity(e).insert(materials.add(bee.clone().into()));
    }
}
