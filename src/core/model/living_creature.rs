pub use bevy::prelude::*;

use super::{BeeType, EnemyType};

#[derive(Component)]
pub struct LivingCreature {
    pub time_alive: f32,

    pub health: i32,

    pub attack_damage: u32,
    pub attack_radius: f32,
    pub attack_cooldown: f32,

    pub time_since_last_attack: f32,
    pub time_since_last_damage_taken: f32,

    pub accumulated_push_back: Vec2,
}

impl Default for LivingCreature {
    fn default() -> Self {
        Self {
            time_alive: Default::default(),
            health: Default::default(),
            attack_damage: Default::default(),
            attack_cooldown: 10.0,
            attack_radius: 14.0,
            time_since_last_attack: Default::default(),
            time_since_last_damage_taken: 1000.,
            accumulated_push_back: Vec2::ZERO,
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
            other.accumulated_push_back +=
                direction.normalize_or_zero() * (self.attack_damage as f32).powf(0.6).min(3.0);
            self.accumulated_push_back -=
                direction.normalize_or_zero() * (self.attack_damage as f32).powf(0.4).min(2.0) * 0.8;
        }
    }
}

impl From<BeeType> for LivingCreature {
    fn from(value: BeeType) -> Self {
        match value {
            BeeType::Baby => LivingCreature {
                health: 1,
                attack_damage: 0,
                attack_cooldown: 10.0,
                ..Default::default()
            },
            BeeType::Regular => LivingCreature {
                health: 2,
                attack_damage: 1,
                attack_cooldown: 1.5,
                ..Default::default()
            },
            BeeType::Worker => todo!(),
            BeeType::Builder => todo!(),
            BeeType::Defender => LivingCreature {
                health: 4,
                attack_damage: 2,
                attack_cooldown: 2.5,
                ..Default::default()
            },
            BeeType::Queen => LivingCreature {
                health: 60,
                attack_damage: 2,
                attack_cooldown: 2.0,
                ..Default::default()
            },
        }
    }
}

impl From<EnemyType> for LivingCreature {
    fn from(value: EnemyType) -> Self {
        match value {
            EnemyType::Wasp => LivingCreature {
                health: 10,
                attack_damage: 1,
                attack_cooldown: 2.0,
                ..Default::default()
            },
            EnemyType::Birb => LivingCreature {
                health: 40,
                attack_damage: 6,
                attack_cooldown: 2.0,
                attack_radius: 24.0,
                ..Default::default()
            },
        }
    }
}
