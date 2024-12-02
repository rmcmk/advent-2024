use std::ops::RangeInclusive;

fn main() {
    p1();
    p2()
}

fn p1() {
    let safe_count = parse_input().into_iter().filter(Report::is_safe).count();
    println!("P1: Safe Reports: {}", safe_count);
}

fn p2() {
    let reports = parse_input();

    let tolerable_safe_count = reports
        .iter()
        .filter(|r| {
            r.is_safe()
                || r.levels.iter().enumerate().any(|(i, _)| {
                    let mut levels = r.levels.clone();
                    levels.remove(i);
                    Report::new(levels).is_safe()
                })
        })
        .count();

    println!("P2: Tolerable-Safe Reports: {}", tolerable_safe_count);
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    const VALID_RANGE: RangeInclusive<i32> = 0..=3;

    fn new(levels: Vec<i32>) -> Report {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        let mut pairs = self.levels.windows(2);
        let mut is_ascending = true;
        let mut is_descending = true;

        for w in pairs.clone() {
            if w[0] >= w[1] {
                is_ascending = false;
            }
            if w[0] <= w[1] {
                is_descending = false;
            }
        }

        let all_safe = pairs.all(|w| {
            let delta = (w[0] - w[1]).abs();
            Self::VALID_RANGE.contains(&delta)
        });

        (is_ascending || is_descending) && all_safe
    }
}

fn parse_input() -> Vec<Report> {
    include_str!("../../input/day02.txt")
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            Report::new(levels)
        })
        .collect()
}
