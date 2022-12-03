use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let compartments = line.split_at(line.len() / 2);
            let items: [HashSet<char>; 2] = [
                HashSet::from_iter(compartments.0.chars()),
                HashSet::from_iter(compartments.1.chars()),
            ];
            let intersection = items[0].intersection(&items[1]).next().unwrap();
            item_priority(*intersection)
        })
        .sum()
}

fn part2(input: &[String]) -> u32 {
    input
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|compartments| -> HashSet<_> { compartments.chars().collect() })
                .reduce(|acc, rucksack| &acc & &rucksack)
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .map(item_priority)
        .sum()
}

fn item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}
