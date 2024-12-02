fn main() {
    p1();
    p2()
}

fn p1() {
    let (mut first_list, mut second_list) = parse_input();
    first_list.sort_unstable();
    second_list.sort_unstable();

    let total_distance: i32 = first_list
        .iter()
        .zip(&second_list)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("P1: Total Distance: {}", total_distance);
}

fn p2() {
    let (first_list, second_list) = parse_input();

    let mut similarity_score = 0;
    first_list.iter().for_each(|a| {
        let occurrences = second_list.iter().filter(|b| a == *b).count() as i32;
        similarity_score += a * occurrences;
    });

    println!("P2: Similarity score: {}", similarity_score);
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    include_str!("../../input/day01.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip()
}
