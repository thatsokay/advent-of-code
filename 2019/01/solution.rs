use std::fs;
use std::iter::successors;

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
    input.iter().fold(0, |acc, mass| acc + fuel(mass))
}

fn part2(input: &Vec<i32>) -> i32 {
    input.iter().fold(0, |acc, mass| acc + fuel_recursive(mass))
}

fn fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn fuel_recursive(mass: &i32) -> i32 {
    successors(Some(*mass), |mass| Some(fuel(mass)))
        .skip(1)
        .take_while(|mass| mass > &0)
        .sum()
}
