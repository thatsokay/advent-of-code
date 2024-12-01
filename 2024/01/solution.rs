use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Input = [Vec<i32>; 2];

fn parse_input() -> Input {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = fs::read_to_string("input.txt")
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
