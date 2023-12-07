use bevy::{
    input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit},
    prelude::*,
};

use crate::utils::FlatProvider;

pub const MAX_VIEW_RECT: Rect = Rect {
    min: Vec2::new(-900.0, -500.0),
    max: Vec2::new(900.0, 500.0),
};
pub const MAX_VIEW_RECT_SOFT: Rect = Rect {
    min: Vec2::new(-850.0, -450.0),
    max: Vec2::new(850.0, 450.0),
};

fn get_view_rect(camera: &Camera, camera_transform: &Transform) -> Rect {
    let matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    let max = matrix.project_point3(Vec2::ONE.extend(-1.0)).flat();
    let min = matrix.project_point3(Vec2::NEG_ONE.extend(-1.0)).flat();

    Rect {
        min: min.min(max),
        max: max.max(min),
    }
}

fn clamp_to_rect(pos: Vec2, view_half_size: Vec2, rect: Rect, factor: f32) -> Vec2 {
    [0, 1]
        .map(|i| {
            if rect.min[i] * factor + view_half_size[i] >= rect.max[i] * factor - view_half_size[i]
            {
                (rect.min[i] * factor + rect.max[i] * factor) * 0.5
            } else {
                pos[i].clamp(
                    rect.min[i] * factor + view_half_size[i],
                    rect.max[i] * factor - view_half_size[i],
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
    mut target_zoom: Local<Option<f32>>,
    time: Res<Time>,
) {
    if *target_zoom == None {
        *target_zoom = Some(1.0);
    }

    let _window = windows.single();

    // max view height
    const MAX_VIEW_HEIGHT: f32 = 600.0;
    const MAX_VIEW_WIDTH: f32 = MAX_VIEW_HEIGHT * 16.0 / 9.0;

    for (camera, mut transform) in cameras.iter_mut() {
        for e in mouse_wheel_events.read() {
            // todo: resolution dependant\
            
            println!("{:?}", e.unit);
            if target_zoom.unwrap() > 0.05 || e.y < 0.0 {
                let scroll_amount = if e.unit == MouseScrollUnit::Line { e.y } else {e.y / 16.0 };
                *target_zoom.as_mut().unwrap() /= 1.1f32.powf(scroll_amount);
            }
        }

        let change = (target_zoom.unwrap() / transform.scale.x).powf(time.delta_seconds() * 10.0);
        transform.scale *= change;

        // LOL
        let view_rect = get_view_rect(camera, &transform);
        let view_size = view_rect.max - view_rect.min;
        if view_size.y > MAX_VIEW_HEIGHT {
            let zoom = MAX_VIEW_HEIGHT / view_size.y;
            transform.scale *= zoom;
            *target_zoom = Some(transform.scale.x);
        }
        let view_rect = get_view_rect(camera, &transform);
        let view_size = view_rect.max - view_rect.min;
        if view_size.x > MAX_VIEW_WIDTH {
            let zoom = MAX_VIEW_WIDTH / view_size.x;
            transform.scale *= zoom;
            *target_zoom = Some(transform.scale.x);
        }
        let view_rect = get_view_rect(camera, &transform);
        let view_size = view_rect.max - view_rect.min;

        let mut pos = transform.flat();

        for e in mouse_motion_events.read() {
            if mouse.pressed(MouseButton::Left) {
                pos.x -= e.delta.x * transform.scale.x;
                pos.y += e.delta.y * transform.scale.y;
            }
        }

        let factor = ((view_size.y / MAX_VIEW_HEIGHT).powf(0.5) * 1.2).min(1.0);

        // Clamp camera to boundaries
        let view_rect = get_view_rect(&camera, &transform);
        let view_size = (view_rect.max - view_rect.min) * 0.5;
        pos = clamp_to_rect(pos, view_size, MAX_VIEW_RECT, factor);
        if !mouse.pressed(MouseButton::Left) {
            pos = Vec2::lerp(
                pos,
                clamp_to_rect(pos, view_size, MAX_VIEW_RECT_SOFT, factor),
                0.05,
            );
        }
        transform.translation = pos.extend(transform.translation.z);
    }
}
