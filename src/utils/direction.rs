use bevy::prelude::Vec2;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Direction {
    Right,
    Top,
    Left,
    Bottom,
}

impl Into<Vec2> for Direction {
    fn into(self) -> Vec2 {
        match self {
            Direction::Right => Vec2::new(1., 0.),
            Direction::Top => Vec2::new(0., 1.),
            Direction::Left => Vec2::new(-1., 0.),
            Direction::Bottom => Vec2::new(0., -1.),
        }
    }
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Top,
            Direction::Left,
            Direction::Bottom,
        ]
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Top => 1,
            Direction::Left => 2,
            Direction::Bottom => 3,
        }
    }

    pub fn from_index(index: u32) -> Direction {
        Self::all()[index as usize]
    }
}
