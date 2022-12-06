use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<char> {
    fs::read_to_string("input.txt").unwrap().chars().collect()
}

fn part1(input: &[char]) -> usize {
    input
        .windows(4)
        .enumerate()
        .find(|(_, window)| HashSet::<char>::from_iter(window.iter().copied()).len() == 4)
        .unwrap()
        .0
        + 4
}

fn part2(input: &[char]) -> usize {
    input
        .windows(14)
        .enumerate()
        .find(|(_, window)| HashSet::<char>::from_iter(window.iter().copied()).len() == 14)
        .unwrap()
        .0
        + 14
}
