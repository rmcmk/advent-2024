use advent_2024::vec2d::direction::Direction;
use advent_2024::vec2d::position::Position2d;
use advent_2024::vec2d::Vec2d;
use std::collections::HashSet;
use std::ops::ControlFlow;
use std::time::Instant;

fn main() {
    let p1_start = Instant::now();
    p1();
    println!("P1 took: {:?}", p1_start.elapsed());

    let p2_start = Instant::now();
    p2();
    println!("P2 took: {:?}", p2_start.elapsed());
}

fn p1() {
    let grid = parse_input();
    let start_position = grid.find_first(|&c| c == GUARD).unwrap();

    let mut distinct_positions: HashSet<Position2d> = HashSet::new();
    find_route(&grid, &start_position, |pos, _| {
        distinct_positions.insert(pos.clone());
        return ControlFlow::Continue(());
    });

    println!("P1: Distinct Visited: {}", distinct_positions.len());
}

fn p2() {
    let grid = parse_input();
    let start_position = grid.find_first(|&c| c == GUARD).unwrap();

    let mut distinct_positions: HashSet<Position2d> = HashSet::new();
    find_route(&grid, &start_position, |pos, _| {
        distinct_positions.insert(pos.clone());
        return ControlFlow::Continue(());
    });

    let possible_grids = distinct_positions
        .iter()
        .map(|pos| {
            let mut grid = grid.clone();
            grid[&pos] = if grid[&pos] == GUARD { GUARD } else { BLOCKED };
            grid
        })
        .collect::<Vec<_>>();

    let mut circular_paths = 0;
    for grid in possible_grids {
        let mut steps: HashSet<Step> = HashSet::new();

        find_route(&grid, &start_position, |pos, dir| {
            if !steps.insert(Step {
                position: pos.clone(),
                direction: dir.clone(),
            }) {
                circular_paths += 1;
                return ControlFlow::Break(());
            }
            return ControlFlow::Continue(());
        });
    }

    println!("P2: Circular Paths: {}", circular_paths);
}

const GUARD: char = '^';
const BLOCKED: char = '#';

#[derive(Debug, PartialEq, Eq, Hash)]
struct Step {
    position: Position2d,
    direction: Direction,
}

fn find_route<F>(grid: &Vec2d<char>, start_position: &Position2d, mut on_step: F)
where
    F: FnMut(&Position2d, &Direction) -> ControlFlow<()>,
{
    let mut direction = Direction::North;
    let mut position = start_position.clone();

    loop {
        let next_position = position.translate(&direction);
        if grid.is_out_of_bounds(&next_position) {
            break;
        }

        if grid[&next_position] == BLOCKED {
            direction = direction.rotate_90_clockwise();
        } else {
            position = next_position;

            if on_step(&position, &direction) == ControlFlow::Break(()) {
                break;
            }
        }
    }
}

fn parse_input() -> Vec2d<char> {
    Vec2d::from_string(include_str!("../../input/day06.txt"))
}
