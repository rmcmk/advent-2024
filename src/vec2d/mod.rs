pub mod direction;
pub mod position;

use crate::vec2d::position::Position2d;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Vec2d<T> {
    pub fn new(width: usize, height: usize, cells: Vec<T>) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "Number of cells ({}) does not match width * height ({} * {})",
            cells.len(),
            width,
            height
        );
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, position: &Position2d) -> Option<&T> {
        if self.is_out_of_bounds(position) {
            None
        } else {
            self.cells.get(position.y * self.width + position.x)
        }
    }

    pub fn find_first<F>(&self, condition: F) -> Option<Position2d>
    where
        F: Fn(&T) -> bool,
    {
        self.cells.iter().enumerate().find_map(|(idx, cell)| {
            if condition(cell) {
                Some(Position2d::new(idx % self.width, idx / self.width))
            } else {
                None
            }
        })
    }

    pub fn is_out_of_bounds(&self, position: &Position2d) -> bool {
        position.x >= self.width || position.y >= self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Vec2d<char> {
    pub fn from_string(string: &str) -> Self {
        let lines: Vec<&str> = string.lines().collect();
        let height = lines.len();
        let width = lines.first().map_or(0, |line| line.len());

        assert!(height > 0, "Input string must contain at least one line");
        assert!(
            lines.iter().all(|line| line.len() == width),
            "All lines must have the same width"
        );

        let cells = lines.iter().flat_map(|line| line.chars()).collect();
        Self::new(width, height, cells)
    }
}

impl<T> Index<&Position2d> for Vec2d<T> {
    type Output = T;

    fn index(&self, position: &Position2d) -> &Self::Output {
        assert!(
            !self.is_out_of_bounds(position),
            "Index out of bounds: ({}, {})",
            position.x,
            position.y
        );
        &self.cells[position.y * self.width + position.x]
    }
}

impl<T> IndexMut<&Position2d> for Vec2d<T> {
    fn index_mut(&mut self, position: &Position2d) -> &mut Self::Output {
        assert!(
            !self.is_out_of_bounds(position),
            "Index out of bounds: ({}, {})",
            position.x,
            position.y
        );
        &mut self.cells[position.y * self.width + position.x]
    }
}
