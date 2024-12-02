use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

type Input = [Vec<i32>; 2];

fn parse_input(file_path: OsString) -> Input {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip();
    left.sort();
    right.sort();
    [left, right]
}

fn part1(input: &Input) -> i32 {
    input[0]
        .iter()
        .zip(input[1].iter())
        .map(|pair| (pair.0 - pair.1).abs())
        .sum()
}

fn part2(input: &Input) -> i32 {
    input[0]
        .iter()
        .map(|left| input[1].iter().filter(|&right| left == right).count() as i32 * left)
        .sum()
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
