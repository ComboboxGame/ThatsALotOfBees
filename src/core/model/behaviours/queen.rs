use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::core::{HiveMap, NavigationResult, NavigationTarget, Bee, BeeKind};

#[derive(Component, Default)]
pub struct QueenBee {}

pub fn queen_bee_system(
    mut bees: Query<(
        &mut Bee,
        &Transform,
        &mut NavigationTarget,
        &NavigationResult,
    ), With<QueenBee>>,
    map: Res<HiveMap>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (mut bee, transform, mut target, result) in bees.iter_mut() {
        if result.is_reached() {
            for _ in 0..32 {
                const AREA_RADIUS: f32 = 100.0;
                const MIN_DISTNACE_TO_NEW_TARGET: f32 = 50.0;
                let x = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                let y = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                if x * x + y * y > AREA_RADIUS * AREA_RADIUS {
                    continue;
                }
                if map.get_obstruction_xy(x, y) > 0.0 {
                    continue;
                }
                let pos = Vec2::new(x, y);
                if transform.translation.truncate().distance(pos) < MIN_DISTNACE_TO_NEW_TARGET {
                    continue;
                }

                *target = NavigationTarget::Position(pos);
            }
        }
    }
}
