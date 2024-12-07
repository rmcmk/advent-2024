use crate::vec2d::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position2d {
    pub x: usize,
    pub y: usize,
}

impl Position2d {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from_tuple((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn translate(&self, direction: &Direction) -> Self {
        let (dx, dy) = direction.to_deltas();
        Self::new(
            (self.x as isize + dx) as usize,
            (self.y as isize + dy) as usize,
        )
    }
}
