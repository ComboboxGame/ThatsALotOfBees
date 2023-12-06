use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component, Default)]
pub struct MaxSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct VelocityOriented;

pub fn movement_system(mut agents: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in agents.iter_mut() {
        transform.translation += velocity.value.extend(0.0) * time.delta_seconds();
    }
}

pub fn movement_orientation_system(
    mut agents: Query<(&Velocity, &mut Transform), With<VelocityOriented>>,
) {
    for (velocity, mut transform) in agents.iter_mut() {
        let delta = velocity.value;
        if delta.length() > 0.01 {
            if transform.scale.x > 0.0 {
                // positive orientation already
                if delta.normalize().x > 0.1 {
                    transform.scale.x = -1.0;
                }
            } else {
                // negative orientation already
                if delta.normalize().x < -0.1 {
                    transform.scale.x = 1.0;
                }
            }

            let angle = if delta.x < 0.0 {
                f32::atan2(-delta.y, -delta.x)
            } else {
                f32::atan2(delta.y, delta.x)
            };
            transform.rotation =
                Quat::from_axis_angle(Vec3::Z, (angle.clamp(-1.0, 1.0) / 0.7).sin() * 0.7);
        }
    }
}
