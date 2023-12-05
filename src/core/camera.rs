use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

fn get_view_rect(camera: &Camera, camera_transform: &Transform) -> Rect {
    let matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    let max = matrix.project_point3(Vec2::ONE.extend(-1.0)).truncate();
    let min = matrix.project_point3(Vec2::NEG_ONE.extend(-1.0)).truncate();

    Rect {
        min: min.min(max),
        max: max.max(min),
    }
}

fn clamp_to_rect(pos: Vec2, view_half_size: Vec2, rect: Rect) -> Vec2 {
    [0, 1]
        .map(|i| {
            if rect.min[i] + view_half_size[i] >= rect.max[i] - view_half_size[i] {
                (rect.min[i] + rect.max[i]) * 0.5
            } else {
                pos[i].clamp(
                    rect.min[i] + view_half_size[i],
                    rect.max[i] - view_half_size[i],
                )
            }
        })
        .into()
}

pub fn in_game_camera_system(
    mut cameras: Query<(&Camera, &mut Transform)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let window = windows.single();

    // max view height
    const MAX_VIEW_HEIGHT: f32 = 320.0;
    const MAX_VIEW_WIDTH: f32 = 512.0;

    const MAX_VIEW_RECT: Rect = Rect {
        min: Vec2::new(-300.0, -228.0),
        max: Vec2::new(300.0, 228.0),
    };
    const MAX_VIEW_RECT_SOFT: Rect = Rect {
        min: Vec2::new(-280.0, -180.0),
        max: Vec2::new(280.0, 180.0),
    };
    for (camera, mut transform) in cameras.iter_mut() {
        for e in mouse_wheel_events.read() {
            transform.scale /= 1.1f32.powf(e.y);
        }

        let view_rect = get_view_rect(camera, &transform);
        let view_size = view_rect.max - view_rect.min;
        if view_size.y > MAX_VIEW_HEIGHT {
            let zoom = MAX_VIEW_HEIGHT / view_size.y;
            transform.scale *= zoom;
        } else if view_size.x > MAX_VIEW_WIDTH {
            let zoom = MAX_VIEW_WIDTH / view_size.x;
            transform.scale *= zoom;
        }

        let mut pos = transform.translation.truncate();

        for e in mouse_motion_events.read() {
            if mouse.pressed(MouseButton::Left) {
                pos.x -= e.delta.x * transform.scale.x;
                pos.y += e.delta.y * transform.scale.y;
            }
        }

        // Clamp camera to boundaries
        let view_rect = get_view_rect(&camera, &transform);
        let view_size = (view_rect.max - view_rect.min) * 0.5;
        pos = clamp_to_rect(pos, view_size, MAX_VIEW_RECT);
        if !mouse.pressed(MouseButton::Left) {
            pos = Vec2::lerp(pos, clamp_to_rect(pos, view_size, MAX_VIEW_RECT_SOFT), 0.05);
        }
        transform.translation = pos.extend(transform.translation.z);
    }
}
