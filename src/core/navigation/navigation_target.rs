use bevy::prelude::*;

use crate::utils::{dist_to_segment, FlatProvider};

use super::{HiveGraph, HIVE_GRAPH_RADIUS};

const REACH_DISTANCE: f32 = 4.0;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum NavigationTarget {
    None,
    Position(Vec2),
    Entity(Entity, f32),
}

#[derive(Component, PartialEq, Clone)]
pub enum Faction {
    Bees,
    Enemies,
}

#[derive(Component, Default, Clone)]
pub struct NavigationResult {
    target_reached: bool,
    last_reached: bool,
    next_path_point: Option<Vec2>,
    next_and_last: Option<(usize, usize)>,
    time_since_refresh: f32,
}

impl NavigationResult {
    pub fn get_direction(&self, my_position: Vec2) -> Vec2 {
        if let Some(to) = self.next_path_point {
            (to - my_position).normalize()
        } else {
            Vec2::ZERO
        }
    }

    pub fn is_reached(&self) -> bool {
        self.target_reached
    }
}

pub fn navigation_system(
    _commands: Commands,
    mut query: Query<(
        Entity,
        &NavigationTarget,
        Changed<NavigationTarget>,
        &Transform,
        &mut NavigationResult,
    )>,
    all_entities: Query<&Transform>,
    graph: Res<HiveGraph>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    keyboard: Res<Input<KeyCode>>,
) {
    if !graph.ready {
        return;
    }

    let seed = (time.elapsed_seconds() * 100.0) as usize;

    // todo: parallel??
    query
        .iter_mut()
        .for_each(|(e, target, target_changed, transform, mut result)| {
            let from = transform.flat();
            let seed = seed + e.index() as usize;
            if target_changed {
                *result = NavigationResult::default();
            }

            match target {
                NavigationTarget::None => {
                    *result = NavigationResult::default();
                    result.target_reached = true;
                }
                NavigationTarget::Position(to) => 'position: {
                    let to = *to;
                    if keyboard.pressed(KeyCode::X) {
                        gizmos.line_2d(from, to, Color::PURPLE);
                    }

                    if from.distance_squared(to) < REACH_DISTANCE.powi(2) || result.target_reached {
                        result.target_reached = true;
                        break 'position;
                    }

                    if dist_to_segment(from, to, Vec2::ZERO) > HIVE_GRAPH_RADIUS * 0.9
                        || from.length() > HIVE_GRAPH_RADIUS * 2.0
                    {
                        result.next_path_point = Some(to);
                        result.next_and_last = None;
                        break 'position;
                    }

                    if let Some((next, last)) = result.next_and_last {
                        if graph.points[next].distance_squared(from) < REACH_DISTANCE.powi(2) {
                            if next == last {
                                result.next_and_last = None;
                                result.next_path_point = Some(to);
                                result.last_reached = true;
                            } else {
                                // assert ? assert!(!result.last_reached);
                                let new_next = graph.get_next(next, last, seed);
                                result.next_and_last = Some((new_next, last));
                                result.next_path_point = Some(graph.points[new_next]);
                            }
                        }
                    } else if !result.last_reached {
                        let nearest_from = graph.get_nearest(from);
                        let nearest_to = graph.get_nearest(to);

                        if nearest_from == nearest_to {
                            result.next_and_last = None;
                            result.next_path_point = Some(to);
                            result.last_reached = true;
                        } else {
                            let next = graph.get_next(nearest_from, nearest_to, seed);
                            result.next_and_last = Some((next, nearest_to));
                            result.next_path_point = Some(graph.points[next]);
                        }
                    }
                }

                NavigationTarget::Entity(e, target_distance) => 'entity: {
                    if let Ok(to) = all_entities.get(*e) {
                        let to = to.flat();
                        if keyboard.pressed(KeyCode::C) {
                            gizmos.line_2d(from, to, Color::RED);
                        }

                        let from_to_sqr = from.distance_squared(to);

                        let should_refresh = if from_to_sqr > 128.0f32.powi(2) {
                            result.time_since_refresh > 1.0 || result.next_path_point == None
                        } else if from_to_sqr > 64.0f32.powi(2) {
                            result.time_since_refresh > 0.5 || result.next_path_point == None
                        } else {
                            result.time_since_refresh > 0.1 || result.next_path_point == None
                        };

                        if should_refresh {
                            if from_to_sqr < (*target_distance * 0.8).powi(2) {
                                result.target_reached = true;
                                break 'entity;
                            }

                            if result.target_reached {
                                if from_to_sqr > (*target_distance).powi(2) {
                                    result.target_reached = false;
                                    result.next_and_last = None;
                                    result.next_path_point = None;
                                }
                                break 'entity;
                            }

                            if dist_to_segment(from, to, Vec2::ZERO) > HIVE_GRAPH_RADIUS * 0.85
                                || from.length() > HIVE_GRAPH_RADIUS * 1.5
                            {
                                result.next_path_point = Some(to);
                                result.next_and_last = None;
                                break 'entity;
                            }

                            if let Some((next, last)) = result.next_and_last {
                                if graph.points[next].distance_squared(from)
                                    < REACH_DISTANCE.powi(2)
                                {
                                    if next == last {
                                        result.next_and_last = None;
                                        result.next_path_point = Some(to);
                                        result.last_reached = true;
                                    } else {
                                        // assert ? assert!(!result.last_reached);
                                        let new_last = graph.get_nearest(to);
                                        let new_next = graph.get_next(next, new_last, seed);
                                        result.next_and_last = Some((new_next, new_last));
                                        result.next_path_point = Some(graph.points[new_next]);
                                    }
                                }
                            } else {
                                let nearest_from = graph.get_nearest(from);
                                let nearest_to = graph.get_nearest(to);

                                if nearest_from == nearest_to {
                                    result.next_and_last = None;
                                    result.next_path_point = Some(to);
                                    result.last_reached = true;
                                } else {
                                    let next = graph.get_next(nearest_from, nearest_to, seed);
                                    result.next_and_last = Some((next, nearest_to));
                                    result.next_path_point = Some(graph.points[next]);
                                }
                            }
                        } else {
                            result.time_since_refresh += time.delta_seconds();
                        }
                    }
                }
            }
        });
}
