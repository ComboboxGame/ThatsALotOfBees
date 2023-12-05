use bevy::prelude::*;

#[derive(Component)]
pub struct NavigationTarget {
    pub target: Option<Vec2>,
}

#[derive(Component)]
pub struct NavigationPathResult {
    pub next_path_point: Option<Vec2>,
}

pub fn navigation_system() {}
