use advent_2024::vec2d::direction::Direction;
use advent_2024::vec2d::position::Position2d;
use advent_2024::vec2d::Vec2d;

fn main() {
    p1();
    p2();
}

fn p1() {
    let grid = parse_input();
    let mut count = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            for direction in Direction::ALL.iter() {
                let position = Position2d::new(x, y);
                if matches_sequence(&grid, position, direction, &XMAS) {
                    count += 1;
                }
            }
        }
    }

    println!("P1: \"XMAS\" occurrences: {}", count);
}

fn p2() {
    let grid = parse_input();
    let mut count = 0;

    for y in 1..(grid.height() - 1) {
        for x in 1..(grid.width() - 1) {
            let position = Position2d::new(x, y);
            if grid[&position] != 'A' {
                continue;
            }

            let diag1 = [
                grid[&position.translate(&Direction::NorthWest)],
                'A',
                grid[&position.translate(&Direction::SouthEast)],
            ];
            let diag2 = [
                grid[&position.translate(&Direction::SouthWest)],
                'A',
                grid[&position.translate(&Direction::NorthEast)],
            ];

            if (diag1 == FORWARD || diag1 == REVERSE) && (diag2 == FORWARD || diag2 == REVERSE) {
                count += 1;
            }
        }
    }

    println!("P2: \"X-MAS\" occurrences: {}", count);
}

fn matches_sequence(
    grid: &Vec2d<char>,
    start: Position2d,
    direction: &Direction,
    sequence: &[char],
) -> bool {
    let mut position = start;
    for &expected in sequence {
        if grid.is_out_of_bounds(&position) || grid[&position] != expected {
            return false;
        }
        position = position.translate(direction);
    }
    true
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const FORWARD: [char; 3] = ['M', 'A', 'S'];
const REVERSE: [char; 3] = ['S', 'A', 'M'];

fn parse_input() -> Vec2d<char> {
    Vec2d::from_string(include_str!("../../input/day04.txt"))
}
