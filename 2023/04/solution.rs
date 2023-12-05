use std::collections::{BTreeMap, HashSet};
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Input = BTreeMap<u32, u32>;

fn parse_input() -> Input {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (card_label, numbers_description) = line.split_once(": ").unwrap();
            let id = card_label
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let number_sets: Vec<_> = numbers_description
                .split(" | ")
                .map(|numbers| {
                    numbers
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect();
            let match_count = number_sets[0]
                .intersection(&number_sets[1])
                .collect::<Vec<_>>()
                .len() as u32;
            (id, match_count)
        })
        .collect()
}

fn part1(input: &Input) -> u32 {
    input
        .values()
        .map(|&match_count| {
            if match_count > 0 {
                2_u32.pow(match_count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .iter()
        .rev()
        .fold(vec![], |acc: Vec<(u32, u32)>, (&id, &match_count)| {
            let copy_count = (0..(match_count)).map(|i| acc[i as usize].1).sum::<u32>() + 1;
            [&[(id, copy_count)], &acc[..]].concat()
        })
        .into_iter()
        .map(|(_id, copy_count)| copy_count)
        .sum()
}
