use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<u32> {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|fish| fish.parse().unwrap())
        .collect()
}

fn part1(input: &[u32]) -> u64 {
    let descendant_count = descendant_count_after_days(80);
    input
        .iter()
        .map(|&fish| descendant_count.get(&fish).unwrap())
        .sum()
}

fn part2(input: &[u32]) -> u64 {
    let descendant_count = descendant_count_after_days(256);
    input
        .iter()
        .map(|&fish| descendant_count.get(&fish).unwrap())
        .sum()
}

fn descendant_count_after_days(days: u32) -> HashMap<u32, u64> {
    fn count_fish(fish: u32, days: u32, memoised: &mut HashMap<(u32, u32), u64>) -> u64 {
        if let Some(&memoised_value) = memoised.get(&(fish, days)) {
            return memoised_value;
        }
        let count = match (fish, days) {
            (_, 0) => 1,
            (0, _) => count_fish(6, days - 1, memoised) + count_fish(8, days - 1, memoised),
            _ => count_fish(fish - 1, days - 1, memoised),
        };
        memoised.insert((fish, days), count);
        count
    }

    let mut memoised: HashMap<(u32, u32), u64> = HashMap::new();
    for fish in 0..=6 {
        count_fish(fish, days, &mut memoised);
    }
    memoised
        .iter()
        .filter(|((_, memoised_days), _)| *memoised_days == days)
        .map(|((fish, _), count)| (*fish, *count))
        .collect()
}
