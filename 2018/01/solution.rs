use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |acc, frequency| acc + frequency)
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut cumulative = 0;
    let mut seen = HashSet::new();
    for frequency in input.iter().cycle() {
        cumulative += frequency;
        if seen.contains(&cumulative) {
            return cumulative;
        }
        seen.insert(cumulative);
    }
    panic!()
}
