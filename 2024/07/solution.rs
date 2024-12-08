use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

type Input = Vec<(u64, Vec<u64>)>;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (
                left.parse().unwrap(),
                right.split(' ').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(test_value, operands)| {
            let mut stack = vec![(operands[0], 1)];
            while let Some((acc, i)) = stack.pop() {
                if i == operands.len() {
                    if acc == *test_value {
                        return true;
                    } else {
                        continue;
                    }
                }
                if acc <= *test_value {
                    stack.push((acc + operands[i], i + 1));
                    stack.push((acc * operands[i], i + 1));
                }
            }
            false
        })
        .map(|(test_value, _)| test_value)
        .sum()
}

fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(test_value, operands)| {
            let mut stack = vec![(operands[0], 1)];
            while let Some((acc, i)) = stack.pop() {
                if i == operands.len() {
                    if acc == *test_value {
                        return true;
                    } else {
                        continue;
                    }
                }
                if acc <= *test_value {
                    stack.push((acc + operands[i], i + 1));
                    stack.push((acc * operands[i], i + 1));
                    stack.push((
                        acc * 10u64.pow(operands[i].ilog10() + 1) + operands[i],
                        i + 1,
                    ));
                }
            }
            false
        })
        .map(|(test_value, _)| test_value)
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
