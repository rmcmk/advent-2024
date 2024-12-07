use Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        North, South, East, West, NorthEast, SouthEast, SouthWest, NorthWest,
    ];

    pub const fn delta_x(&self) -> isize {
        match &self {
            West => -1,
            East => 1,
            _ => 0,
        }
    }

    pub const fn delta_y(&self) -> isize {
        match &self {
            North => -1,
            South => 1,
            _ => 0,
        }
    }

    pub const fn to_deltas(&self) -> (isize, isize) {
        match &self {
            North | South | East | West => (self.delta_x(), self.delta_y()),
            NorthEast => (East.delta_x(), North.delta_y()),
            SouthEast => (East.delta_x(), South.delta_y()),
            SouthWest => (West.delta_x(), South.delta_y()),
            NorthWest => (West.delta_x(), North.delta_y()),
        }
    }

    pub fn from_deltas(dx: isize, dy: isize) -> Self {
        match (dx, dy) {
            (0, -1) => North,
            (0, 1) => South,
            (-1, 0) => West,
            (1, 0) => East,
            (-1, -1) => NorthWest,
            (1, -1) => NorthEast,
            (1, 1) => SouthEast,
            (-1, 1) => SouthWest,
            _ => panic!("Invalid deltas: ({}, {})", dx, dy),
        }
    }

    pub const fn rotate_90_clockwise(&self) -> Self {
        match &self {
            North => East,
            East => South,
            South => West,
            West => North,
            NorthEast => SouthEast,
            SouthEast => SouthWest,
            SouthWest => NorthWest,
            NorthWest => NorthEast,
        }
    }
}
