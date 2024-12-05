use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    p1();
    p2();
}

fn p1() {
    let (rules, updates) = parse_input();

    let middle_page_sum: i32 = updates
        .iter()
        .filter(|update| is_sorted(update, &rules))
        .map(|update| find_middle(update))
        .sum();

    println!("P1: Middle-Page Sum: {}", middle_page_sum);
}

fn p2() {
    let (rules, updates) = parse_input();

    let middle_page_sum: i32 = updates
        .iter()
        .filter_map(|update| {
            if is_sorted(update, &rules) {
                return None;
            }

            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| {
                if is_sorted_pair(*a, *b, &rules) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            Some(find_middle(&sorted_update))
        })
        .sum();

    println!("P2: Middle-Page Sum: {}", middle_page_sum);
}

fn is_sorted(update: &[i32], rules: &Rules) -> bool {
    update
        .windows(2)
        .all(|pair| is_sorted_pair(pair[0], pair[1], rules))
}

fn is_sorted_pair(a: i32, b: i32, rules: &Rules) -> bool {
    rules.get(&a).map_or(false, |related| related.contains(&b))
}

fn find_middle(numbers: &[i32]) -> i32 {
    numbers[numbers.len() / 2]
}

type Rules = HashMap<i32, Vec<i32>>;
type Updates = Vec<Vec<i32>>;

fn parse_input() -> (Rules, Updates) {
    let mut rules: Rules = HashMap::new();
    let mut updates: Updates = Vec::new();

    for line in include_str!("../../input/day05.txt").lines() {
        if line.is_empty() {
            continue;
        }

        if let Some((a, b)) = line.split_once("|") {
            let key = a.trim().parse().unwrap();
            let value = b.trim().parse().unwrap();
            rules.entry(key).or_default().push(value);
        } else {
            let update = line
                .split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    (rules, updates)
}
