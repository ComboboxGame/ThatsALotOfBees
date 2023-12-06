use bevy::prelude::*;
use delaunator::{triangulate, Point};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{hive_map::*, precomputed::DISTANCES};

pub const HIVE_GRAPH_POINTS_NUMBER: usize = 512;
pub const HIVE_GRAPH_RADIUS: f32 = 180.0;
pub const HIVE_GRAPH_INNER_RADIUS: f32 = 160.0;

#[derive(Resource, Default)]
pub struct HiveGraph {
    pub ready: bool,
    pub points: Vec<Vec2>,
    pub adjacent_points: Vec<Vec<usize>>,
    pub next_points: Vec<Vec<Vec<(usize, f32)>>>,
    pub hull: Vec<usize>,
    pub rng: Option<StdRng>,
    pub distances: Vec<Vec<f32>>,
}

#[derive(Clone)]
pub struct CrazyWrapper(*mut Vec<Vec<f32>>);

unsafe impl Send for CrazyWrapper {}

impl HiveGraph {
    pub fn get_next(&mut self, from: usize, to: usize) -> usize {
        if let Some(rng) = self.rng.as_mut() {
            let mut r = rng.gen_range(0.0..1.0);
            for (next, p) in self.next_points[from][to].iter() {
                r -= p;
                if r <= 0.0 {
                    return *next;
                }
            }
        }
        0
    }

    pub fn get_nearest(&self, p: Vec2) -> usize {
        let mut nearest = 0;
        for i in 1..HIVE_GRAPH_POINTS_NUMBER {
            if self.points[i].distance(p) < self.points[nearest].distance(p) {
                nearest = i;
            }
        }
        nearest
    }
}

pub fn build_hive_graph_system(
    hive_map: Res<HiveMap>,
    mut hive_graph: ResMut<HiveGraph>,
    _gizmos: Gizmos,
) {
    if hive_graph.ready {
        for _p in &hive_graph.points {
            //gizmos.circle_2d(*p, 2.0, Color::GREEN);
        }

        for i in 0..HIVE_GRAPH_POINTS_NUMBER {
            for _j in hive_graph.adjacent_points[i].iter() {
                //gizmos.line_2d(hive_graph.points[i], hive_graph.points[*j], Color::BLUE);
            }
        }
    }
    if hive_graph.ready || !hive_map.ready {
        return;
    }

    hive_graph.ready = true;

    let mut rng = StdRng::seed_from_u64(1);

    while hive_graph.points.len() < HIVE_GRAPH_POINTS_NUMBER {
        let mut furthest_point = Vec2::ZERO;
        let mut furthest_point_dist = 0.0;

        for _ in 0..48 {
            let x = rng.gen_range(-HIVE_GRAPH_RADIUS..HIVE_GRAPH_RADIUS);
            let y = rng.gen_range(-HIVE_GRAPH_RADIUS..HIVE_GRAPH_RADIUS);
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

    for _i in 0..HIVE_GRAPH_POINTS_NUMBER {
        let adjacent: Vec<usize> = vec![];
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

    let distance = &DISTANCES;

    /*let mut distance = vec![];
    for _ in 0..HIVE_GRAPH_POINTS_NUMBER {
        distance.push(vec![1e9; HIVE_GRAPH_POINTS_NUMBER]);
    }

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        distance[i][i] = 0.0;
        for j in hive_graph.adjacent_points[i].iter() {
            distance[i][*j] = points[i].distance(points[*j]);
        }
    }*/

    //let distance_ptr: *mut Vec<Vec<f32>> = &mut distance;
    /*for k in 0..HIVE_GRAPH_POINTS_NUMBER {
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
                if distance[i][j] > distance[i][k] + distance[k][j] {
                    distance[i][j] = distance[i][k] + distance[k][j];
                }
            }
        }
    }*/

    /*let mut file: std::fs::File = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("my-file")
        .unwrap();

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        write!(file, "vec![");
        for j in 0..HIVE_GRAPH_POINTS_NUMBER {
            write!(file, "{},", distance[i][j]);
        }
        writeln!(file, "],");
    }*/

    /*for k in 0..HIVE_GRAPH_POINTS_NUMBER {
    }
    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        for j in 0..HIVE_GRAPH_POINTS_NUMBER {
            let mut nearest_point = hive_graph.adjacent_points[i][0];

            for k in hive_graph.adjacent_points[i].iter() {
                if distance[i][*k] + distance[*k][j] < distance[i][nearest_point] + distance[nearest_point][j]
                {
                    nearest_point = *k;
                }
            }

            let diff = (distance[i][nearest_point] + distance[nearest_point][j]) - distance[i][j];
            if diff.abs() > 0.001 {
                println!("Omg diff: {}-{}-{}    {}", i, nearest_point, j, diff);
            }
        }
    }*/

    for i in 0..HIVE_GRAPH_POINTS_NUMBER {
        let mut next_points = vec![];
        for f in 0..HIVE_GRAPH_POINTS_NUMBER {
            let mut next_points_local = vec![];

            if i == f {
                next_points.push(next_points_local);
                continue;
            }

            if hive_graph.adjacent_points[i].len() == 1 {
                next_points_local.push((hive_graph.adjacent_points[i][0], 1.0));
            } else if hive_graph.adjacent_points[i].len() > 1 {
                let mut nearest_point = hive_graph.adjacent_points[i][0];
                let mut nearest_point_next = hive_graph.adjacent_points[i][1];

                if distance[i][nearest_point] + distance[nearest_point][f]
                    > distance[i][nearest_point_next] + distance[nearest_point_next][f]
                {
                    let tmp = nearest_point;
                    nearest_point = nearest_point_next;
                    nearest_point_next = tmp;
                }

                for j in hive_graph.adjacent_points[i].iter() {
                    if distance[i][*j] + distance[*j][f]
                        < distance[i][nearest_point] + distance[nearest_point][f]
                    {
                        nearest_point_next = nearest_point;
                        nearest_point = *j;
                    } else if distance[i][*j] + distance[*j][f]
                        < distance[i][nearest_point_next] + distance[nearest_point_next][f]
                    {
                        nearest_point_next = *j;
                    }
                    /*if distance[i][*j] + distance[*j][f] < distance[i][nearest_point] + distance[nearest_point][f]
                    {
                        nearest_point = *j;
                    }*/
                }

                //next_points_local.push((nearest_point, 1.0));
                next_points_local.push((nearest_point, 0.6));
                next_points_local.push((nearest_point_next, 0.4));
            }

            next_points.push(next_points_local);
        }
        hive_graph.next_points.push(next_points);
    }

    hive_graph.hull = res.hull;

    hive_graph.rng = Some(StdRng::seed_from_u64(0));

    //hive_graph.distances = distance;
}
