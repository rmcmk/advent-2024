fn main() {
    p1();
    p2();
}

fn p1() {
    let (first_list, second_list) = parse_lists();

    let total_distance: i64 = first_list
        .iter()
        .zip(&second_list)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("P1: Total Distance: {}", total_distance);
}

fn p2() {
    let (first_list, second_list) = parse_lists();

    let mut similarity_score: i64 = 0;
    first_list.iter().for_each(|a| {
        let occurrences = second_list.iter().filter(|b| a == *b).count();
        similarity_score += a * occurrences as i64;
    });

    println!("P2: Similarity score: {}", similarity_score);
}

fn parse_lists() -> (Vec<i64>, Vec<i64>) {
    let input = include_str!("../../input/day01.txt");
    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    for line in input.lines() {
        let locations: Vec<_> = line.split_whitespace().collect();
        let first = locations[0].parse::<i64>().unwrap();
        let second = locations[1].parse::<i64>().unwrap();

        first_list.push(first);
        second_list.push(second);
    }

    assert_eq!(first_list.len(), second_list.len());

    first_list.sort_unstable();
    second_list.sort_unstable();

    (first_list, second_list)
}
