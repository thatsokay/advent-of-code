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

fn part1(input: &[i32]) -> i32 {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count() as i32
}

fn part2(input: &[i32]) -> i32 {
    input
        .windows(4)
        .filter(|window| window[0] < window[3])
        .count() as i32
}
