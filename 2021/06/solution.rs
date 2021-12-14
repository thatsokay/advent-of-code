use std::fs;

type FishCounts = [u64; 9];

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> FishCounts {
    let mut fish_counts: FishCounts = [0; 9];
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|days| days.parse::<usize>().unwrap())
        .for_each(|days| fish_counts[days] += 1);
    fish_counts
}

fn part1(input: &FishCounts) -> u64 {
    breed_fish(input, 80)
}

fn part2(input: &FishCounts) -> u64 {
    breed_fish(input, 256)
}

fn breed_fish(counts: &FishCounts, days: u32) -> u64 {
    let mut result = counts.clone();
    for _ in 0..days {
        result.rotate_left(1);
        result[6] += result[8];
    }
    result.iter().sum()
}
