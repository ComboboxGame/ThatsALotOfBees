
use bevy::{ecs::component::Component, math::Vec2, transform::components::Transform};
use std::fmt::Debug;
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
}

#[derive(Debug)]
struct ComputedCollider<E> {
    pub entity: E,
    pub min: Vec2,
    pub max: Vec2,
}

fn collide<E>(a: &ComputedCollider<E>, b: &ComputedCollider<E>) -> bool {
    overlap(a.min.y, a.max.y, b.min.y, b.max.y) > 0.0
}

impl<E: PartialEq> PartialOrd for ComputedCollider<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.max.x.partial_cmp(&other.max.x)
    }
}

impl<E: PartialEq> PartialEq for ComputedCollider<E> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<E: PartialEq> Eq for ComputedCollider<E> {}

impl<E: PartialEq + Ord> Ord for ComputedCollider<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut r = self.max.x.total_cmp(&other.max.x);
        if r == Ordering::Equal {
            r = self.max.y.total_cmp(&other.max.y);
            if r == Ordering::Equal {
                r = self.entity.cmp(&other.entity);
            }
        }
        r
    }
}

pub(super) fn broad_phase<'a, E: PartialEq + Copy + Debug + Ord>(
    colliders: impl Iterator<Item = (E, &'a Collider, &'a Transform)>,
) -> Vec<(E, E)> {
    let mut computed_colliders = colliders
        .map(|(entity, collider, transform)| ComputedCollider {
            entity,
            min: transform.translation.truncate()
                - Vec2::new(collider.width, collider.height) / 1.9,
            max: transform.translation.truncate()
                + Vec2::new(collider.width, collider.height) / 1.9,
        })
        .collect::<Vec<_>>();

    computed_colliders.sort_by(|a, b| a.min.x.total_cmp(&b.min.x));

    let mut sliding = BTreeSet::<ComputedCollider<E>>::new();

    let mut pairs = vec![];

    for collider in computed_colliders.drain(..) {
        while !sliding.is_empty() && sliding.first().unwrap().max.x <= collider.min.x {
            sliding.pop_first();
        }

        for other_collider in sliding.iter() {
            if collide(other_collider, &collider) {
                pairs.push((collider.entity, other_collider.entity));
            }
        }

        sliding.insert(collider);
    }

    pairs
}

pub struct Collision {
    pub point: Vec2,
    pub normal: Vec2,
    pub depth: f32,
}

impl Collision {
    pub fn reversed(&self) -> Self {
        Collision {
            point: self.point,
            normal: -self.normal,
            depth: self.depth,
        }
    }
}

fn overlap(a0: f32, a1: f32, b0: f32, b1: f32) -> f32 {
    a1.min(b1) - a0.max(b0)
}

fn get_bounds_along_dir(a: &Collider, t: &Vec2, dir: &Vec2) -> (f32, f32) {
    let mut left = t.dot(*dir);
    let mut right = left;

    for dx in [-1., 1.] {
        for dy in [-1., 1.] {
            let x = t.x + (a.width / 2. - a.corner_radius) * dx;
            let y = t.y + (a.height / 2. - a.corner_radius) * dy;
            let v = Vec2::new(x, y).dot(*dir);
            left = left.min(v - a.corner_radius);
            right = right.max(v + a.corner_radius);
        }
    }

    (left, right)
}

pub fn get_collision(
    a: &Collider,
    at: &Transform,
    b: &Collider,
    bt: &Transform,
) -> Option<Collision> {
    let directions = [
        Vec2::new(1., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(-1., 1.),
    ]
    .map(|v| v.normalize());

    let mut smallest_overlap = 1e9;
    let mut smallest_overlap_dir = Vec2::Y;

    for dir in directions {
        let (al, ar) = get_bounds_along_dir(a, &at.translation.truncate(), &dir);
        let (bl, br) = get_bounds_along_dir(b, &bt.translation.truncate(), &dir);
        let overlap = overlap(al, ar, bl, br);
        if overlap <= 0.0 {
            return None;
        }

        if overlap < smallest_overlap {
            smallest_overlap = overlap;
            smallest_overlap_dir = dir;
        }
    }

    let ab = (bt.translation - at.translation).truncate();

    Some(Collision {
        normal: if ab.dot(smallest_overlap_dir) >= 0.0 {
            1.
        } else {
            -1.
        } * smallest_overlap_dir,
        depth: smallest_overlap,
        point: (at.translation + bt.translation).truncate() / 2.0,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn sort_pairs<E: PartialEq + Ord + Copy>(a: &Vec<(E, E)>) -> Vec<(E, E)> {
        let mut v: Vec<_> = a.iter().map(|(a, b)| (*a.min(b), *a.max(b))).collect();
        v.sort();
        v
    }

    fn assert_eq_pairs<E: PartialEq + Ord + Copy + Debug>(a: Vec<(E, E)>, b: Vec<(E, E)>) {
        assert_eq!(sort_pairs(&a), sort_pairs(&b));
    }

    #[test]
    fn test_simple() {
        let boxes = vec![
            (
                0,
                Collider {
                    width: 10.0,
                    height: 10.0,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(5.0, 0.0, 0.0),
            ),
            (
                1,
                Collider {
                    width: 10.0,
                    height: 10.0,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(-4.0, 1.0, 0.0),
            ),
            (
                2,
                Collider {
                    width: 2.0,
                    height: 2.0,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(-9.0, 6.9, 0.0),
            ),
            (
                3,
                Collider {
                    width: 0.7,
                    height: 0.7,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.5, 5.2, 0.0),
            ),
            (
                4,
                Collider {
                    width: 25.0,
                    height: 0.2,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.2, 1.3, 0.0),
            ),
        ];

        let pairs = broad_phase(boxes.iter().map(|(a, b, c)| (*a, b, c)));

        assert_eq_pairs(pairs, vec![(1, 2), (0, 1), (3, 1), (3, 0), (0, 4), (1, 4)]);
    }

    #[test]
    fn test_stacked() {
        let boxes = vec![
            (
                0,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            (
                1,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.0, 1.0, 0.0),
            ),
            (
                2,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.0, 2.0, 0.0),
            ),
            (
                3,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.0, 3.0, 0.0),
            ),
        ];

        let pairs = broad_phase(boxes.iter().map(|(a, b, c)| (*a, b, c)));

        assert_eq_pairs(pairs, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_stacked_horizontal() {
        let boxes = vec![
            (
                0,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            (
                1,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(1.0, 0.0, 0.0),
            ),
            (
                2,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(2.0, 0.0, 0.0),
            ),
            (
                3,
                Collider {
                    width: 1.01,
                    height: 1.01,
                    corner_radius: 0.0,
                },
                Transform::from_xyz(3.0, 0.0, 0.0),
            ),
        ];

        let pairs = broad_phase(boxes.iter().map(|(a, b, c)| (*a, b, c)));

        assert_eq_pairs(pairs, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_rounded_corners() {
        let ac = Collider {
            width: 1.0,
            height: 1.0,
            corner_radius: 0.03,
        };
        let at = Transform::from_xyz(0.0, 0.0, 0.0);
        let bc = Collider {
            width: 1.0,
            height: 1.0,
            corner_radius: 0.03,
        };
        let bt = Transform::from_xyz(0.99, 0.99, 0.0);

        assert!(get_collision(&ac, &at, &bc, &bt).is_none())
    }
}
