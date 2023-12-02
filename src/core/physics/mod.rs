mod collision;
mod solver;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    ecs::{
        schedule::{
            common_conditions::{in_state}, IntoSystemConfigs,
        },
        system::Resource,
    },
    math::Vec2,
};
pub use collision::*;
pub use solver::*;

use super::AppState;

pub struct PhysicsPlugin;

#[derive(Resource)]
pub struct PhysicsSettings {
    pub gravity: Vec2,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PhysicsSettings {
            gravity: Vec2::new(0.0, -10.0),
        });
        app.insert_resource(ContactCache::default());
        app.add_systems(Update, physics.run_if(in_state(AppState::InGame)));
        app.add_systems(PreUpdate, update_rigid_body_internal.run_if(in_state(AppState::InGame)));
    }
}
