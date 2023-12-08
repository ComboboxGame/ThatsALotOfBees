use bevy::prelude::*;

mod hive_graph;
mod hive_map;
mod navigation_target;
mod precomputed;

pub use hive_graph::*;
pub use hive_map::*;
pub use navigation_target::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<HiveMap>();
        app.init_resource::<HiveGraph>();
        app.add_systems(PreUpdate, build_hive_map_system);
        app.add_systems(PreUpdate, build_hive_graph_system);
        app.add_systems(Update, navigation_system);
    }
}
