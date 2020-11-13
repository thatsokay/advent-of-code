use std::collections::{HashMap, HashSet};
use std::fs;

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

fn part1(input: &Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for id in input.iter() {
        let (two, three) = has_2_and_3_of_kind(id);
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }
    twos * threes
}

fn part2(input: &Vec<String>) -> String {
    let input_len = input.len();
    for i in 0..input_len {
        for j in i + 1..input_len {
            let common = common_chars(&input[i], &input[j]);
            if common.len() == input[i].len() - 1 {
                return common;
            }
        }
    }
    panic!()
}

fn has_2_and_3_of_kind(id: &String) -> (bool, bool) {
    let mut char_counts = HashMap::new();
    for c in id.chars() {
        char_counts
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let unique_counts: HashSet<i32> = char_counts.values().cloned().collect();
    (unique_counts.contains(&2), unique_counts.contains(&3))
}

fn common_chars(id1: &String, id2: &String) -> String {
    id1.chars()
        .zip(id2.chars())
        .filter_map(|(c, d)| if c == d { Some(c) } else { None })
        .collect()
}
