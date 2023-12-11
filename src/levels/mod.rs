use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{thread_rng, Rng};

use crate::core::{
    EnemyType, Faction, LivingCreature, MoveToNavigationTargetBehaviour, NavigationResult,
    NavigationTarget, RigidBody, SmartOrientation, UniversalBehaviour, MAX_VIEW_RECT, GameInfo, CurrencyValues,
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

#[derive(Component)]
pub struct NextWave;

pub fn scenario0_system(
    mut scenarios: Query<(Entity, &mut Scenario0)>,
    enemis: Query<&EnemyType>,
    time: Res<Time>,
    game: Res<GameInfo>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_wave: Query<&mut Text, With<NextWave>>,
) {
    if game.paused {
        return;
    }
    if scenarios.is_empty() {
        return;
    }

    let (_, mut scenario) = scenarios.single_mut();

    if enemis.is_empty() {
        for mut text in next_wave.iter_mut() {
            text.sections[0].value = format!("Next wave in {}...", 40 - scenario.time_elapsed as i32);
        }
        scenario.time_elapsed += time.delta_seconds();
    } else {
        for mut text in next_wave.iter_mut() {
            text.sections[0].value = format!("Wave {}", scenario.wave);
        }
    }

    if scenario.time_elapsed < 40.0 {
        return;
    }

    scenario.time_elapsed = 0.0;

    let waves = vec![
        vec![
            (EnemyType::Wasp(0), [16, 2, 1]),
        ],
        vec![
            (EnemyType::Wasp(0), [14, 2, 1]),
            (EnemyType::Wasp(0), [14, 2, 1]),
        ],
        vec![
            (EnemyType::Wasp(0), [12, 2, 2]),
            (EnemyType::Wasp(0), [12, 2, 1]),
            (EnemyType::Wasp(0), [12, 2, 2]),
        ],
        vec![
            (EnemyType::Wasp(0), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 3]),
            (EnemyType::Wasp(1), [10, 2, 3]),
        ],
        vec![
            (EnemyType::Birb(0), [40, 16, 8]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
        ],
        vec![
            (EnemyType::Birb(0), [40, 16, 8]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
        ],
        vec![
            (EnemyType::Bumble(0), [40, 16, 8]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
        ],
        vec![
            (EnemyType::Birb(1), [40, 8, 8]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
        ],
        vec![
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
        ],
        vec![
            (EnemyType::Bumble(1), [20, 8, 8]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
        ],
        vec![
            (EnemyType::Birb(2), [20, 8, 8]),
            (EnemyType::Birb(2), [20, 8, 8]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
        ],
        vec![
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
        ],
        vec![
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Birb(2), [20, 8, 8]),
            (EnemyType::Birb(2), [20, 8, 8]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
        ],
        vec![
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Bumble(2), [20, 4, 8]),
            (EnemyType::Birb(2), [20, 4, 8]),
            (EnemyType::Birb(2), [20, 4, 8]),
            (EnemyType::Birb(2), [20, 2, 2]),
            (EnemyType::Birb(2), [20, 2, 2]),
            (EnemyType::Bumble(1), [20, 2, 2]),
            (EnemyType::Bumble(1), [20, 2, 2]),
            (EnemyType::Birb(1), [20, 2, 2]),
            (EnemyType::Birb(1), [20, 2, 2]),
            (EnemyType::Bumble(0), [20, 2, 2]),
            (EnemyType::Bumble(0), [20, 2, 2]),
            (EnemyType::Birb(0), [20, 2, 2]),
            (EnemyType::Birb(0), [20, 2, 2]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
            (EnemyType::Wasp(1), [10, 2, 1]),
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

fn spawn_enemy(commands: &mut Commands, enemy: EnemyType, meshes: &mut Assets<Mesh>, drop: CurrencyValues) {
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
            currency_drop: drop,
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
