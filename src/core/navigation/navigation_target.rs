use std::ops::Deref;

use bevy::prelude::*;

use crate::utils::dist_to_segment;

use super::{HiveGraph, HIVE_GRAPH_RADIUS};

const NEXT_PATH_POINT_MIN_DISTANCE: f32 = 4.0;

#[derive(Component)]
pub enum NavigationTarget {
    None,
    Position(Vec2),
    Entity(Entity),
}

#[derive(Component, Default, Clone)]
pub struct NavigationResult {
    target_reached: bool,
    last_reached: bool,
    next_path_point: Option<Vec2>,
    next_and_last: Option<(usize, usize)>,
}

impl NavigationResult {
    pub fn get_direction(&self, my_position: Vec2) -> Vec2 {
        if let Some(to) = self.next_path_point {
            (to - my_position).normalize()
        } else {
            Vec2::ZERO
        }
    }
}

pub fn navigation_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &NavigationTarget,
        Changed<NavigationTarget>,
        &Transform,
        Option<&mut NavigationResult>,
    )>,
    mut graph: ResMut<HiveGraph>,
    mut gizmos: Gizmos,
) {
    for (e, target, target_changed, transform, maybe_result) in query.iter_mut() {
        let from = transform.translation.truncate();
        let mut result = maybe_result
            .as_ref()
            .map_or(NavigationResult::default(), |r| {
                if target_changed {
                    NavigationResult::default()
                } else {
                    r.deref().clone()
                }
            });

        match target {
            NavigationTarget::None => {
                result = NavigationResult::default();
            }
            NavigationTarget::Position(to) => 'position: {
                if dist_to_segment(from, *to, Vec2::ZERO) > HIVE_GRAPH_RADIUS * 0.9
                    || from.length() > HIVE_GRAPH_RADIUS * 2.0
                {
                    result.next_path_point = Some(*to);
                    result.next_and_last = None;
                    break 'position;
                }

                if let Some((next, last)) = result.next_and_last {
                    if graph.points[next].distance(from) < NEXT_PATH_POINT_MIN_DISTANCE {
                        if next == last {
                            result.next_and_last = None;
                            result.next_path_point = Some(*to);
                            result.last_reached = true;
                        } else {
                            // assert ? assert!(!result.last_reached);
                            let new_next = graph.get_next(next, last);
                            result.next_and_last = Some((new_next, last));
                            result.next_path_point = Some(graph.points[new_next]);
                        }
                    }
                } else if !result.last_reached {
                    let nearest_from = graph.get_nearest(from);
                    let nearest_to = graph.get_nearest(*to);

                    if nearest_from == nearest_to {
                        result.next_and_last = None;
                        result.next_path_point = Some(*to);
                        result.last_reached = true;
                    } else {
                        let next = graph.get_next(nearest_from, nearest_to);
                        result.next_and_last = Some((next, nearest_to));
                        result.next_path_point = Some(graph.points[next]);
                    }
                }
            }
            NavigationTarget::Entity(_) => {}
        }

        if let Some(mut some_result) = maybe_result {
            *some_result = result;
        } else {
            commands.entity(e).insert(result);
        }
    }
}
