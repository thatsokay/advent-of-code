use std::cmp::Ordering;
use std::collections::HashSet;
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
