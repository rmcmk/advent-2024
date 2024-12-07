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
    let calibrations = parse_input();
    let result = evaluate(&calibrations, &[Operator::Add, Operator::Multiply]);
    println!("P1: Total calibration result: {}", result);
}

fn p2() {
    let calibrations = parse_input();
    let result = evaluate(
        &calibrations,
        &[Operator::Add, Operator::Multiply, Operator::Concatenate],
    );
    println!("P2: Total calibration result: {}", result);
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn evaluate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                format!("{}{}", lhs_str, rhs_str).parse::<usize>().unwrap()
            }
        }
    }
}

#[derive(Debug)]
struct BridgeCalibration {
    test_value: usize,
    numbers: Vec<usize>,
}

impl BridgeCalibration {
    fn is_valid(&self, operators: &[Operator]) -> bool {
        let total_operators = self.numbers.len() - 1;
        let combinations = operators.len().pow(total_operators as u32); // len(operators)^(n-1)

        for i in 0..combinations {
            let mut result = self.numbers[0];
            let mut temp = i;

            for (_, &num) in self.numbers.iter().enumerate().skip(1) {
                let op = &operators[temp % operators.len()];
                temp /= operators.len();
                result = op.evaluate(result, num);
            }

            if result == self.test_value {
                return true;
            }
        }

        false
    }
}

fn evaluate(calibrations: &[BridgeCalibration], operators: &[Operator]) -> usize {
    calibrations
        .iter()
        .filter(|c| c.is_valid(operators))
        .map(|c| c.test_value)
        .sum()
}

fn parse_input() -> Vec<BridgeCalibration> {
    include_str!("../../input/day07.txt")
        .lines()
        .map(|line| {
            let (test_value_str, numbers_str) = line.split_once(":").unwrap();
            let test_value = test_value_str.trim().parse::<usize>().unwrap();
            let numbers = numbers_str
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            BridgeCalibration {
                test_value,
                numbers,
            }
        })
        .collect()
}
