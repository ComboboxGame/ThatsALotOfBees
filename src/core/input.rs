use bevy::prelude::*;

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
}

fn update_mouse_state_system(
    mut mouse_state: ResMut<MouseState>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &Transform)>,
    mut mouse_motion_events: EventReader<CursorMoved>,
) {
    let window = windows.single();
    let camera = cameras.single();

    if let Some(e) = mouse_motion_events.read().last() {
        let x = e.position.x - window.width() / 2.0;
        let y = window.height() / 2.0 - e.position.y;
        mouse_state.position =
            Some(camera.1.translation.truncate() + Vec2::new(x, y) * camera.1.scale.truncate());
    }
}