pub use bevy::prelude::*;

use crate::core::NavigationTarget;

use super::{BeeType, EnemyType, RigidBody, UniversalMaterial, CurrencyValue, CurrencyValues, CurrencyStorage};

#[derive(Component)]
pub struct LivingCreature {
    pub time_alive: f32,

    pub max_health: i32,
    pub health: i32,

    pub attack_damage: u32,
    pub attack_radius: f32,
    pub attack_cooldown: f32,

    pub time_since_last_attack: f32,
    pub time_since_last_damage_taken: f32,

    pub accumulated_push_back: Vec2,

    pub currency_drop: CurrencyValues,
    pub end_game_on_dead: bool,
}

impl Default for LivingCreature {
    fn default() -> Self {
        Self {
            time_alive: Default::default(),
            health: Default::default(),
            max_health: Default::default(),
            attack_damage: Default::default(),
            attack_cooldown: 10.0,
            attack_radius: 14.0,
            time_since_last_attack: Default::default(),
            time_since_last_damage_taken: 1000.,
            accumulated_push_back: Vec2::ZERO,
            currency_drop: CurrencyValues::default(),
            end_game_on_dead: false,
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
        if !other.is_dead() && self.attack_damage > 0 && self.can_attack() {
            other.health -= self.attack_damage as i32;
            self.time_since_last_attack = 0.0;
            other.time_since_last_damage_taken = 0.0;

            let pb = direction.normalize_or_zero() * (self.attack_damage as f32 / other.max_health as f32).min(1.0).powf(0.6).min(3.0);

            other.accumulated_push_back += pb;
            self.accumulated_push_back -= pb * 0.2;
        }
    }
}

impl From<BeeType> for LivingCreature {
    fn from(value: BeeType) -> Self {
        match value {
            BeeType::Baby => LivingCreature {
                health: 1,
                max_health: 1,
                attack_damage: 0,
                attack_cooldown: 0.0,
                ..Default::default()
            },
            BeeType::Regular => LivingCreature {
                health: 2,
                max_health: 2,
                attack_damage: 1,
                attack_cooldown: 1.5,
                ..Default::default()
            },
            BeeType::Worker(lvl) => LivingCreature {
                //todo:
                health: 2 + 3 * lvl as i32,
                max_health: 2 + 3 * lvl as i32,
                attack_damage: 0,
                attack_cooldown: 2.0,
                ..Default::default()
            },
            BeeType::Defender(lvl) => LivingCreature {
                health: 5 + 5 * lvl as i32,
                max_health: 5 + 5 * lvl as i32,
                attack_damage: 2 + 1 * lvl,
                attack_cooldown: 2.5 - 0.4 * lvl as f32,
                ..Default::default()
            },
            BeeType::Queen => LivingCreature {
                health: 100,
                max_health: 100,
                attack_damage: 4,
                attack_cooldown: 1.5,
                end_game_on_dead: true,
                ..Default::default()
            },
        }
    }
}

impl From<EnemyType> for LivingCreature {
    fn from(value: EnemyType) -> Self {
        match value {
            EnemyType::Wasp(lvl) => LivingCreature {
                health: 8 * (lvl + 1) as i32,
                max_health:  8 * (lvl + 1) as i32,
                attack_damage: 1 + lvl,
                attack_cooldown: 2.0,
                ..Default::default()
            },
            EnemyType::Birb(lvl) => LivingCreature {
                health: [40, 100, 240][lvl as usize],
                max_health:  [40, 100, 240][lvl as usize],
                attack_damage: [8, 12, 16][lvl as usize],
                attack_cooldown: 2.2,
                attack_radius: 28.0,
                ..Default::default()
            },
            EnemyType::Bumble(lvl) => LivingCreature {
                health: [80, 160, 400][lvl as usize],
                max_health:  [80, 160, 400][lvl as usize],
                attack_damage: [16, 20, 40][lvl as usize],
                attack_cooldown: 4.5,
                attack_radius: 28.0,
                ..Default::default()
            },
        }
    }
}

#[derive(Resource, Default)]
pub struct GameInfo {
    pub end: bool,
    pub paused: bool,
}

pub fn living_creature_system(
    mut creatures: Query<(
        Entity,
        &mut LivingCreature,
        Option<&Handle<UniversalMaterial>>,
        Option<&mut RigidBody>,
    )>,
    mut targets: Query<&mut NavigationTarget>,
    time: Res<Time>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    mut commands: Commands,
    mut storage: ResMut<CurrencyStorage>,
    mut game_end: ResMut<GameInfo>,
) {
    if game_end.paused {
        return;
    }
    for (e, mut creature, maybe_material, maybe_rb) in creatures.iter_mut() {
        if creature.time_since_last_damage_taken == 0.0 {
            if let Some(material) = maybe_material {
                if let Some(material) = materials.get_mut(material) {
                    material.props.damage_time = time.elapsed_seconds();
                }
            }
        }

        creature.time_alive += time.delta_seconds();
        creature.time_since_last_attack += time.delta_seconds();
        creature.time_since_last_damage_taken += time.delta_seconds();

        if let Some(mut rb) = maybe_rb {
            rb.velocity += creature.accumulated_push_back * 40.0;

            if creature.is_dead() {
                rb.velocity += Vec2::new(0.0, -90.0) * time.delta_seconds();
            }
        }

        creature.accumulated_push_back = Vec2::ZERO;

        if creature.is_dead() && creature.time_since_last_damage_taken > 0.8 {
            for i in 0..3 {
                storage.stored[i] = (storage.stored[i] + creature.currency_drop[i]).min(storage.max_stored[i]);
            }
            commands.entity(e).despawn();
            if creature.end_game_on_dead {
                game_end.end = true;
            }
        }
    }

    // Clear targets to dead living creatures
    for mut target in targets.iter_mut() {
        if let NavigationTarget::Entity(e, _) = *target {
            if let Ok((_, creature, _, _)) = creatures.get(e) {
                if creature.is_dead() {
                    *target = NavigationTarget::None;
                }
            } else {
                *target = NavigationTarget::None;
            }
        }
    }
}
