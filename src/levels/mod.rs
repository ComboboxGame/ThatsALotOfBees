use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{thread_rng, Rng};

use crate::core::{
    LivingCreature, MaxSpeed, MoveToNavigationTargetBehaviour, NavigationResult, NavigationTarget,
    Velocity, VelocityOriented, WaspKind, MAX_VIEW_RECT,
};
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scenario0_system);
    }
}

#[derive(Component, Default)]
pub struct Scenario0 {
    pub time_elapsed: f32,
    pub wave: u32,
}

pub fn scenario0_system(
    mut scenarios: Query<(Entity, &mut Scenario0)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if scenarios.is_empty() {
        return;
    }

    let (_, mut scenario) = scenarios.single_mut();

    scenario.time_elapsed += time.delta_seconds();

    if scenario.time_elapsed > 1.0 && scenario.wave == 0 {
        scenario.wave += 1;
        spawn_wasp(&mut commands, WaspKind::Regular, &mut meshes);
    }
}

fn spawn_wasp(commands: &mut Commands, wasp: WaspKind, meshes: &mut Assets<Mesh>) {
    let dx = thread_rng().gen_range(-1.0..1.0);
    let dy = thread_rng().gen_range(-1.0..1.0);
    let z = thread_rng().gen_range(0.0..1.0);

    let t0 = if dx > 0.0 {
        MAX_VIEW_RECT.max.x / dx
    } else {
        MAX_VIEW_RECT.min.x / dx
    };
    let t1 = if dy > 0.0 {
        MAX_VIEW_RECT.max.y / dy
    } else {
        MAX_VIEW_RECT.min.y / dy
    };
    let t = t0.min(t1) / 2.0;

    let position = Vec3::new(dx * t, dy * t, z);
    commands.spawn((
        VisibilityBundle::default(),
        TransformBundle::from_transform(Transform::from_translation(position)),
        Mesh2dHandle(meshes.add(Quad::new(Vec2::splat(24.0)).into())),
        wasp,
        LivingCreature::from(wasp),
        NavigationTarget::None,
        NavigationResult::default(),
        MoveToNavigationTargetBehaviour,
        Velocity::default(),
        VelocityOriented,
        MaxSpeed { value: 64.0 },
    ));
}
