use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{thread_rng, Rng};

use crate::core::{
    EnemyType, Faction, LivingCreature, MoveToNavigationTargetBehaviour, NavigationResult,
    NavigationTarget, RigidBody, SmartOrientation, UniversalBehaviour, MAX_VIEW_RECT,
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
    pub wave: usize,
}

pub fn scenario0_system(
    mut scenarios: Query<(Entity, &mut Scenario0)>,
    enemis: Query<&EnemyType>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if scenarios.is_empty() {
        return;
    }

    let (_, mut scenario) = scenarios.single_mut();

    if enemis.is_empty() {
        scenario.time_elapsed += time.delta_seconds();
    }

    if scenario.time_elapsed < 60.0 {
        return;
    }

    scenario.time_elapsed = 0.0;

    let waves = vec![
        vec![
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
        ],
        vec![
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
        ],
        vec![
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
        ],
        vec![
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
        ],
        vec![
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
            (EnemyType::Wasp(0), 0),
        ],
    ];

    for (enemy, drop) in waves[scenario.wave].iter() {
        spawn_enemy(&mut commands, *enemy, &mut meshes, *drop);
    }

    if scenario.wave + 1 < waves.len() {
        scenario.wave += 1;
    }
    
}

fn get_size(value: EnemyType) -> f32 {
    match value {
        EnemyType::Wasp(_) => 24.0,
        EnemyType::Birb(_)=> 48.0,
        EnemyType::Bumble(_)=> 48.0,
    }
}

fn spawn_enemy(commands: &mut Commands, enemy: EnemyType, meshes: &mut Assets<Mesh>, drop: u32) {
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
        Mesh2dHandle(meshes.add(Quad::new(Vec2::splat(get_size(enemy))).into())),
        enemy,
        LivingCreature {
            currency_drop: [0, 0, drop as u64],
            ..LivingCreature::from(enemy)
        },
        RigidBody::from(enemy),
        UniversalBehaviour::from(enemy),
        NavigationTarget::None,
        NavigationResult::default(),
        MoveToNavigationTargetBehaviour,
        SmartOrientation,
        Faction::Enemies,
    ));
}
