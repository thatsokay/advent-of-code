use std::fs;
use std::iter::once;

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

fn part1(input: &[i32]) -> i32 {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count() as i32
}

fn part2(input: &[i32]) -> i32 {
    let cumsum: Vec<i32> = once(&0)
        .chain(input.iter())
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();
    (4..cumsum.len())
        .filter(|&i| cumsum[i] - cumsum[i - 3] > cumsum[i - 1] - cumsum[i - 4])
        .count() as i32
}
