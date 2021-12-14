use std::collections::HashMap;
use std::fs;

type Rules = HashMap<[char; 2], char>;
type Polymer = HashMap<[char; 2], u64>;

struct Polymerise {
    rules: Rules,
    polymer: Polymer,
    first_element: char,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Polymerise {
    let input = fs::read_to_string("input.txt").unwrap();
    let (polymer_raw, rules_raw) = input.split_once("\n\n").unwrap();
    let rules: Rules = rules_raw
        .lines()
        .map(|line| {
            let (pattern, insert) = line.split_once(" -> ").unwrap();
            let pattern_chars: Vec<char> = pattern.chars().collect();
            (
                [pattern_chars[0], pattern_chars[1]],
                insert.chars().next().unwrap(),
            )
        })
        .collect();
    let mut polymer: Polymer = HashMap::new();
    for window in polymer_raw.chars().collect::<Vec<char>>().windows(2) {
        *polymer.entry([window[0], window[1]]).or_insert(0) += 1;
    }
    let first_element = input.chars().next().unwrap();
    Polymerise {
        rules,
        polymer,
        first_element,
    }
}

fn part1(input: &Polymerise) -> u64 {
    element_count_difference(input, 10)
}

fn part2(input: &Polymerise) -> u64 {
    element_count_difference(input, 40)
}

fn step(rules: &Rules, polymer: &Polymer) -> Polymer {
    let mut polymerised: Polymer = HashMap::new();
    for (pair, count) in polymer.iter() {
        match rules.get(pair) {
            Some(created) => {
                *polymerised.entry([pair[0], *created]).or_insert(0) += count;
                *polymerised.entry([*created, pair[1]]).or_insert(0) += count;
            }
            None => {
                *polymerised.entry(*pair).or_insert(0) += count;
            }
        }
    }
    polymerised
}

fn element_count_difference(polymerise: &Polymerise, steps: u32) -> u64 {
    let polymerised = (0..steps).fold(polymerise.polymer.clone(), |acc, _| {
        step(&polymerise.rules, &acc)
    });
    let mut elements: HashMap<char, u64> = HashMap::new();
    for (pair, count) in polymerised {
        *elements.entry(pair[1]).or_insert(0) += count;
    }
    *elements.entry(polymerise.first_element).or_insert(0) += 1;
    let min = elements.values().min().unwrap();
    let max = elements.values().max().unwrap();
    max - min
}
