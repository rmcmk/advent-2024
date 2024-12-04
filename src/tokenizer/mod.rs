use std::iter::Peekable;
use std::str::Chars;

pub enum TokenMatch<'a> {
    Instruction(&'a str),
    Symbol(char),
    Number(i64),
}

pub enum Token<'a> {
    Instruction(&'a str),
    Symbol(char),
    Number,
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn skip(&mut self) {
        self.chars.next();
    }

    pub fn has_next(&mut self) -> bool {
        self.chars.peek().is_some()
    }

    pub fn next(&mut self, tokens: &[Token<'a>]) -> Option<Vec<TokenMatch<'a>>> {
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