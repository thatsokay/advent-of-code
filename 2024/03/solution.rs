use std::env;
use std::ffi::OsString;
use std::fs;
use std::iter::repeat;
use std::process;
use std::str::Chars;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

type Input = String;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path).unwrap()
}

#[derive(Debug, Clone)]
struct Program<'a>(Chars<'a>);

impl<'a> Program<'a> {
    fn from(source: &'a str) -> Self {
        Self(source.chars())
    }

    fn advance(&mut self) -> Option<char> {
        self.0.next()
    }

    fn parse_string(&mut self, pattern: &str) -> Option<()> {
        if self.0.clone().take(pattern.len()).collect::<String>() == pattern {
            self.0.nth(pattern.len() - 1);
            Some(())
        } else {
            None
        }
    }

    fn parse_digits(&mut self) -> Option<u32> {
        let mut chars = self.0.clone();
        let mut digits = vec![];
        while let Some(c) = chars.next() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
            } else {
                break;
            }
        }
        if digits.len() > 0 {
            self.0.nth(digits.len() - 1);
            Some(digits.into_iter().fold(0, |acc, d| acc * 10 + d))
        } else {
            None
        }
    }

    fn parse_mul(&mut self) -> Option<u32> {
        self.parse_string("mul(")
            .and_then(|_| self.parse_digits())
            .and_then(|a| {
                self.parse_string(",")
                    .and_then(|_| self.parse_digits())
                    .and_then(|b| self.parse_string(")").and(Some(a * b)))
            })
    }
}

fn part1(input: &Input) -> u32 {
    let mut program = Program::from(input);
    let mut result = 0;
    loop {
        if let Some(n) = program.parse_mul() {
            result += n;
        } else if let None = program.advance() {
            break;
        }
    }
    result
}

fn part2(input: &Input) -> u32 {
    let mut program = Program::from(input);
    let mut enabled = true;
    let mut result = 0;
    loop {
        if let Some(_) = program.parse_string("do()") {
            enabled = true;
        } else if let Some(_) = program.parse_string("don't()") {
            enabled = false;
        } else if let Some(n) = program.parse_mul() {
            if enabled {
                result += n;
            }
        } else if let None = program.advance() {
            break;
        }
    }
    result
}
