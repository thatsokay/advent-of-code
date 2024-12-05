use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

type PageOrderings = HashSet<[u32; 2]>;
type Updates = Vec<Vec<u32>>;
type Input = (PageOrderings, Updates);

fn parse_input(file_path: OsString) -> Input {
    let contents = fs::read_to_string(file_path).unwrap();
    let sections = contents.split_once("\n\n").unwrap();
    let orderings = sections
        .0
        .lines()
        .map(|line| {
            let mut numbers = line.split('|').map(|s| s.parse().unwrap());
            [numbers.next().unwrap(), numbers.next().unwrap()]
        })
        .collect();
    let updates = sections
        .1
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    (orderings, updates)
}

fn part1(input: &Input) -> u32 {
    let mut updates = input.1.clone();
    for update in updates.iter_mut() {
        update.sort_by(|&a, &b| {
            if input.0.contains(&[a, b]) {
                Ordering::Less
            } else if input.0.contains(&[b, a]) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        println!("{:?}", update);
    }
    updates
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &Input) -> usize {
    0
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
