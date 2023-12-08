use bevy::{
    math::{Vec2, Vec3},
    transform::components::{GlobalTransform, Transform},
};

pub fn cross2d(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn dist_to_segment(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    let t = (c - a).dot((b - a).normalize()) / a.distance(b);
    if t <= 0.0 {
        (a - c).length()
    } else if t >= 1.0 {
        (b - c).length()
    } else {
        (cross2d(a - c, b - c) / a.distance(b)).abs()
    }
}

pub trait FlatProvider {
    fn flat(&self) -> Vec2;
}

impl FlatProvider for Vec3 {
    fn flat(&self) -> Vec2 {
        self.truncate()
    }
}

impl FlatProvider for Transform {
    fn flat(&self) -> Vec2 {
        self.translation.flat()
    }
}

impl FlatProvider for GlobalTransform {
    fn flat(&self) -> Vec2 {
        self.translation().flat()
    }
}
