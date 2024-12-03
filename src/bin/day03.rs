use std::iter::Peekable;
use std::str::Chars;

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

enum TokenMatch<'a> {
    Instruction(&'a str),
    Symbol(char),
    Number(i64),
}

enum Token<'a> {
    Instruction(&'a str),
    Symbol(char),
    Number,
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip(&mut self) {
        self.chars.next();
    }

    fn has_next(&mut self) -> bool {
        self.chars.peek().is_some()
    }

    fn next(&mut self, tokens: &[Token<'a>]) -> Option<Vec<TokenMatch<'a>>> {
        let mut matches = Vec::new();
        let mut chars_clone = self.chars.clone();

        for token in tokens {
            match token {
                Token::Instruction(s) => {
                    let mut temp = String::new();

                    for _ in 0..s.len() {
                        if let Some(ch) = chars_clone.next() {
                            temp.push(ch);
                        } else {
                            return None;
                        }
                    }

                    if temp == *s {
                        matches.push(TokenMatch::Instruction(s));
                    } else {
                        return None;
                    }
                }

                Token::Symbol(c) => {
                    if let Some(&ch) = chars_clone.peek() {
                        if ch == *c {
                            chars_clone.next();
                            matches.push(TokenMatch::Symbol(ch));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                Token::Number => {
                    let mut num_str = String::new();

                    while let Some(&ch) = chars_clone.peek() {
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                            chars_clone.next();
                        } else {
                            break;
                        }
                    }

                    if !num_str.is_empty() {
                        if let Ok(number) = num_str.parse::<i64>() {
                            matches.push(TokenMatch::Number(number));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
        }

        self.chars = chars_clone;
        Some(matches)
    }
}

fn parse_input<'a>() -> &'a str {
    include_str!("../../input/day03.txt")
}
