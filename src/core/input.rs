use bevy::prelude::*;

use crate::utils::FlatProvider;

pub struct InputHelperPlugin;

impl Plugin for InputHelperPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseState>();
        app.add_systems(PreUpdate, update_mouse_state_system);
    }
}

#[derive(Resource, Default)]
pub struct MouseState {
    pub position: Option<Vec2>,
    pub screen_position: Option<Vec2>,
}

fn update_mouse_state_system(
    mut mouse_state: ResMut<MouseState>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &Transform)>,
    mut mouse_motion_events: EventReader<CursorMoved>,
) {
    // Only one window and only one camera
    if let Some(window) = windows.iter().next() {
        let camera = cameras.single();

        if let Some(e) = mouse_motion_events.read().last() {
            let x = e.position.x - window.width() / 2.0;
            let y = window.height() / 2.0 - e.position.y;
            let translation = camera.1.flat();
            let scale = camera.1.scale.truncate();
            mouse_state.screen_position = Some(e.position);
            mouse_state.position = Some(translation + Vec2::new(x, y) * scale);
        }
    }
}
