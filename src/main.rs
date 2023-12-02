use std::env;

use bevy::prelude::*;

mod core;
mod utils;

use crate::core::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugins(CorePlugin);

    if env::var("LOCAL_BUILD") == Ok("2".to_string()) {
    } else {
    }

    app.run();
}
