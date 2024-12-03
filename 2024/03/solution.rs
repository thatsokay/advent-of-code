use std::env;
use std::ffi::OsString;
use std::fs;
use std::iter::repeat;
use std::process;
use std::str::Chars;

type Input = String;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path).unwrap()
}

#[derive(Debug, Clone)]
struct Program<'a> {
    source: Chars<'a>,
}

impl Program<'_> {
    fn advance(&mut self) -> Option<char> {
        self.source.next()
    }

    fn parse_string(&mut self, pattern: &str) -> Option<()> {
        if self
            .source
            .clone()
            .map(|x| Some(x))
            .chain(repeat(None))
            .zip(pattern.chars())
            .all(|(a_option, b)| a_option.and_then(|a| Some(a == b)).unwrap_or(false))
        {
            self.source.nth(pattern.len() - 1);
            Some(())
        } else {
            None
        }
    }

    fn parse_digits(&mut self) -> Option<u32> {
        let mut safe_source = self.source.clone();
        let mut digits = vec![];
        while let Some(c) = safe_source.next() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
            } else {
                break;
            }
        }
        if digits.len() > 0 {
            self.source.nth(digits.len() - 1);
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
    let mut program = Program {
        source: input.chars(),
    };
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
    let mut program = Program {
        source: input.chars(),
    };
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

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, String> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    match get_first_arg() {
        Ok(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
