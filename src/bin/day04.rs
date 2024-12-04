struct CharGrid {
    cells: Vec<char>,
    rows: usize,
    cols: usize,
}

impl CharGrid {
    fn new(cells: Vec<char>, cols: usize) -> Self {
        let rows = cells.len() / cols;
        CharGrid { cells, rows, cols }
    }

    fn get(&self, row: i32, col: i32) -> Option<char> {
        if row >= 0 && (row as usize) < self.rows && col >= 0 && (col as usize) < self.cols {
            Some(self.cells[(row as usize) * self.cols + (col as usize)])
        } else {
            None
        }
    }
}

fn main() {
    p1();
    p2();
}

const DIRECTION_X: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DIRECTION_Y: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn p1() {
    let input = parse_input();
    let cols = input.lines().next().unwrap().chars().count();
    let cells: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let grid = CharGrid::new(cells, cols);

    let xmas_chars: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    for row in 0..grid.rows as i32 {
        for col in 0..grid.cols as i32 {
            for dir in 0..DIRECTION_X.len() {
                if check_word(&grid, row, col, DIRECTION_X[dir], DIRECTION_Y[dir], &xmas_chars) {
                    count += 1;
                }
            }
        }
    }

    println!("P1: \"XMAS\" occurrences: {}", count);
}

fn check_word(grid: &CharGrid, x: i32, y: i32, delta_x: i32, delta_y: i32, word_chars: &[char]) -> bool {
    for (i, &ch) in word_chars.iter().enumerate() {
        let r = x + delta_x * i as i32;
        let c = y + delta_y * i as i32;

        match grid.get(r, c) {
            Some(cell_ch) if cell_ch == ch => continue,
            _ => return false,
        }
    }

    true
}

fn p2() {
    let input = parse_input();
    let cols = input.lines().next().unwrap().chars().count();
    let cells: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let grid = CharGrid::new(cells, cols);

    let forward = [Some('M'), Some('A'), Some('S')];
    let reverse = [Some('S'), Some('A'), Some('M')];

    let mut count = 0;

    for row in 1..(grid.rows as i32 - 1) {
        for col in 1..(grid.cols as i32 - 1) {
            let middle = grid.get(row, col);
            if middle != Some('A') {
                continue;
            }

            let diag1 = [
                grid.get(row - 1, col - 1),
                middle,
                grid.get(row + 1, col + 1),
            ];

            let diag2 = [
                grid.get(row - 1, col + 1),
                middle,
                grid.get(row + 1, col - 1),
            ];

            if (diag1 == forward || diag1 == reverse) && (diag2 == forward || diag2 == reverse) {
                count += 1;
            }
        }
    }

    println!("P2: X-MAS occurrences: {}", count);
}

fn parse_input() -> &'static str {
    include_str!("../../input/day04.txt")
}
