use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

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

type Input = Vec<Vec<i32>>;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| {
            [1, -1]
                .iter()
                .map(|ordering| {
                    report
                        .windows(2)
                        .map(|window| (window[0] - window[1]) * ordering)
                        .all(|difference| difference >= 1 && difference <= 3)
                })
                .any(|safe| safe)
        })
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| {
            [1, -1]
                .iter()
                .flat_map(|ordering| {
                    (0..report.len()).map(move |skip_index| {
                        (0..report.len())
                            .filter(|&index| index != skip_index)
                            .collect::<Vec<usize>>()
                            .windows(2)
                            .map(|indeces| (report[indeces[0]] - report[indeces[1]]) * ordering)
                            .all(|difference| 1 <= difference && difference <= 3)
                    })
                })
                .any(|safe| safe)
        })
        .count()
}
