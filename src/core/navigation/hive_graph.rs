use bevy::prelude::*;
use delaunator::{triangulate, Point};
use rand::Rng;

use crate::utils::{cross2d, dist_to_segment};

use super::hive_map::*;

const HIVE_GRAPH_POINTS_NUMBER: usize = 512;
const HIVE_GRAPH_RADIUS: f32 = 180.0;
const HIVE_GRAPH_INNER_RADIUS: f32 = 128.0;
const MAX_EDGE_LENGTH: f32 = 16.0;

#[derive(Resource, Default)]
pub struct HiveGraph {
    pub ready: bool,
    pub points: Vec<Vec2>,
    pub adjacent_points: Vec<Vec<usize>>,
    pub next_points: Vec<Vec<Vec<(usize, f32)>>>,
    pub hull: Vec<usize>,
}

impl HiveGraph {}

#[derive(Clone)]
pub struct CrazyWrapper(*mut Vec<Vec<f32>>);

unsafe impl Send for CrazyWrapper {}

impl HiveGraph {
    pub fn get_direction(&self, from: Vec2, to: Vec2) -> Vec2 {
        if dist_to_segment(from, to, Vec2::ZERO) > HIVE_GRAPH_RADIUS || from.length() > HIVE_GRAPH_RADIUS * 2.0 {
            // simple case, just go towards the target
            return (to - from).normalize()
        }
        
        if from.length() > HIVE_GRAPH_RADIUS {
            // find start point
        }

        if to.length() > HIVE_GRAPH_RADIUS {
            // find finish point
        }

        return Vec2::ZERO;
    }

    fn get_nearest_points(&self, v: Vec2) -> Vec<usize> {
        if v.length() > HIVE_GRAPH_RADIUS {

        } else {
            
        }
        vec![]
    }
}

pub fn build_hive_graph_system(
    hive_map: Res<HiveMap>,
    mut hive_graph: ResMut<HiveGraph>,
    mut gizmos: Gizmos,
) {
    if hive_graph.ready {
        for p in &hive_graph.points {
            gizmos.circle_2d(*p, 2.0, Color::GREEN);
        }

        for i in 0..HIVE_GRAPH_POINTS_NUMBER {
            for j in hive_graph.adjacent_points[i].iter() {
                gizmos.line_2d(hive_graph.points[i], hive_graph.points[*j], Color::BLUE);
            }
        }
    }
    if hive_graph.ready || !hive_map.ready {
        return;
    }

    hive_graph.ready = true;

    while hive_graph.points.len() < HIVE_GRAPH_POINTS_NUMBER {
        let mut furthest_point = Vec2::ZERO;
        let mut furthest_point_dist = 0.0;

        for _ in 0..48 {
            let x = rand::thread_rng().gen_range(-HIVE_GRAPH_RADIUS..HIVE_GRAPH_RADIUS);
            let y = rand::thread_rng().gen_range(-HIVE_GRAPH_RADIUS..HIVE_GRAPH_RADIUS);
            if x * x + y * y >= HIVE_GRAPH_RADIUS.powi(2) {
                continue;
            }
            if hive_map.get_obstruction_xy(x, y) > 0.0 {
                continue;
            }
            let mut closest_dist = 1e9f32;
            for other_point in &hive_graph.points {
                closest_dist = closest_dist.min(Vec2::new(x, y).distance(*other_point));
            }

            //let dst = (x*x + y*y).sqrt();
            //let coefficient = 1.0 / (1.0 + ((dst - HIVE_GRAPH_INNER_RADIUS).max(0.0) / (HIVE_GRAPH_RADIUS - HIVE_GRAPH_INNER_RADIUS)).powf(1.5));
            //closest_dist = closest_dist * coefficient;

            if closest_dist > furthest_point_dist {
                furthest_point_dist = closest_dist;
                furthest_point = Vec2::new(x, y);
            }
        }

        if furthest_point_dist > 0.0 {
            hive_graph.points.push(furthest_point);
        }
    }

    hive_graph.adjacent_points.reserve(HIVE_GRAPH_POINTS_NUMBER);

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        let mut adjacent: Vec<usize> = vec![];
        /*for j in 0..i {
            let dist = hive_graph.points[i].distance(hive_graph.points[j]);
            if dist > MAX_EDGE_LENGTH {
                continue;
            }
            adjacent.push(j);
            hive_graph.adjacent_points[j].push(i);
        }*/
        hive_graph.adjacent_points.push(adjacent);
    }

    let res = triangulate(
        &hive_graph
            .points
            .clone()
            .drain(..)
            .map(|v| Point {
                x: v.x as f64,
                y: v.y as f64,
            })
            .collect::<Vec<_>>()[..],
    );

    let points = hive_graph.points.clone();
    let any_obstacles = |i: usize, j: usize| -> bool {
        let u = points[i];
        let v = points[j];
        for f in [0.1, 0.2, 0.5, 0.8, 0.9] {
            let o = u * f + v * (1.0 - f);
            if hive_map.get_obstruction(o) > 0.1 {
                return true;
            }
        }
        false
    };

    for i in 0..res.triangles.len() / 3 {
        let a = res.triangles[i * 3];
        let b = res.triangles[i * 3 + 1];
        let c = res.triangles[i * 3 + 2];
        if !any_obstacles(a, b) {
            hive_graph.adjacent_points[a].push(b);
        }
        if !any_obstacles(b, c) {
            hive_graph.adjacent_points[b].push(c);
        }
        if !any_obstacles(c, a) {
            hive_graph.adjacent_points[c].push(a);
        }
    }

    for i in 0..res.hull.len() {
        let a = res.hull[i];
        let b = res.hull[if i == 0 { res.hull.len() - 1 } else { i - 1 }];
        if !any_obstacles(a, b) {
            hive_graph.adjacent_points[a].push(b);
        }
    }

    let mut distance = vec![];
    for _ in 0..HIVE_GRAPH_POINTS_NUMBER {
        distance.push(vec![1e9; HIVE_GRAPH_POINTS_NUMBER]);
    }

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        for j in hive_graph.adjacent_points[i].iter() {
            distance[i][*j] = points[i].distance(points[*j]);
        }
    }

    //let distance_ptr: *mut Vec<Vec<f32>> = &mut distance;
    for k in 0..HIVE_GRAPH_POINTS_NUMBER {
        /*
        bevy::tasks::ComputeTaskPool::get().scope(|scope| {
            for f in 0..16 {
                let left = HIVE_GRAPH_POINTS_NUMBER / 16 * f;
                let right = HIVE_GRAPH_POINTS_NUMBER / 16 * (f + 1);

                let distance_ptr_clone = CrazyWrapper(distance_ptr.clone());

                scope.spawn(async move {
                    let v = distance_ptr_clone.clone();
                    unsafe {
                        let distance = &*(v.0);
                        for i in left..right {
                            for j in 0..HIVE_GRAPH_POINTS_NUMBER {
                                if distance[i][j] > distance[i][k] + distance[k][j] {
                                }
                            }
                        }
                    }
                });
            }
        });*/

        for i in 0..HIVE_GRAPH_POINTS_NUMBER {
            for j in 0..HIVE_GRAPH_POINTS_NUMBER {
                if distance[i][j] > distance[i][k] + distance[k][j] {}
            }
        }
    }

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        let mut next_points = vec![];
        for f in 0..HIVE_GRAPH_POINTS_NUMBER {
            let mut next_points_local = vec![];

            if hive_graph.adjacent_points[i].len() == 1 {
                next_points_local.push((hive_graph.adjacent_points[i][0], 1.0));
            } else if hive_graph.adjacent_points[i].len() > 1 {
                let mut nearest_point = hive_graph.adjacent_points[i][0];
                let mut nearest_point_next = hive_graph.adjacent_points[i][1];
                if distance[nearest_point][f] > distance[nearest_point_next][f] {
                    let tmp = nearest_point;
                    nearest_point = nearest_point_next;
                    nearest_point_next = tmp;
                }

                for j in &hive_graph.adjacent_points[i][2..] {
                    if distance[*j][f] < distance[nearest_point][f] {
                        nearest_point_next = nearest_point;
                        nearest_point = *j;
                    } else if distance[*j][f] < distance[nearest_point_next][f] {
                        nearest_point_next = *j;
                    }
                }

                next_points_local.push((nearest_point, 0.6));
                next_points_local.push((nearest_point_next, 0.4));
            }

            next_points.push(next_points_local);
        }
        hive_graph.next_points.push(next_points);
    }

    hive_graph.hull = res.hull;
}
