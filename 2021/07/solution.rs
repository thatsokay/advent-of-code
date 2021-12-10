use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input: &[i32]) -> i32 {
    let max_x = input.iter().max().unwrap();
    (0..=*max_x)
        .map(|x| input.iter().map(|crab_x| (crab_x - x).abs()).sum())
        .min()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    let max_x = input.iter().max().unwrap();
    (0..=*max_x)
        .map(|x| {
            input
                .iter()
                .map(|crab_x| {
                    let distance = (crab_x - x).abs();
                    distance * (distance + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
