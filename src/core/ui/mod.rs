use bevy::prelude::*;
use self::counter::update_counter;

mod constants;
mod counter;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, counter::setup_bee_counters);

        app.add_systems(Update, update_counter);
    }
}

