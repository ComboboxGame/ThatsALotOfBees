use self::counter::update_counter;
use bevy::prelude::*;

use super::BeeMaterial;

mod constants;
mod counter;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiMaterialPlugin::<BeeMaterial>::default());

        app.add_systems(Startup, counter::setup_bee_counters);

        app.add_systems(Update, update_counter);
    }
}
