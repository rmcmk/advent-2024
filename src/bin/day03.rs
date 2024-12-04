use advent_2024::tokenizer::{Token, TokenMatch, Tokenizer};

fn main() {
    p1();
    p2();
}

const MUL_PATTERN: &[Token; 6] = &[
    Token::Instruction("mul"),
    Token::Symbol('('),
    Token::Number,
    Token::Symbol(','),
    Token::Number,
    Token::Symbol(')'),
];

const PAUSE_TOKEN: Token = Token::Instruction("don't()");
const RESUME_TOKEN: Token = Token::Instruction("do()");

fn p1() {
    let mut tokenizer = Tokenizer::new(parse_input());
    let total = process_tokens(&mut tokenizer, |_| true);

    println!("P1: Uncorrupted Sum: {}", total);
}

fn p2() {
    let mut tokenizer = Tokenizer::new(parse_input());
    let mut active = true;

    let total = process_tokens(&mut tokenizer, |tokenizer| {
        if tokenizer.next(&[RESUME_TOKEN]).is_some() {
            active = true;
        }
        if tokenizer.next(&[PAUSE_TOKEN]).is_some() {
            active = false;
        }
        active
    });

    println!("P2: Uncorrupted Active Sum: {}", total);
}

fn process_tokens<F>(tokenizer: &mut Tokenizer, mut condition: F) -> i64
where
    F: FnMut(&mut Tokenizer) -> bool,
{
    let mut total = 0;

    while tokenizer.has_next() {
        if !condition(tokenizer) {
            tokenizer.skip();
            continue;
        }

        let matches = tokenizer.next(MUL_PATTERN);
        if matches.is_none() {
            tokenizer.skip();
            continue;
        }

        if let [_, _, TokenMatch::Number(n1), _, TokenMatch::Number(n2), _] =
            matches.unwrap().as_slice()
        {
            total += n1 * n2;
        }
    }

    total
}

fn parse_input<'a>() -> &'a str {
    include_str!("../../input/day03.txt")
}
