use std::collections::{HashMap, HashSet};
use std::fs;

type Cave = HashMap<(i32, i32), u8>;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Cave {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(move |(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap() as u8))
        })
        .collect()
}

fn part1(input: &Cave) -> u32 {
    input
        .iter()
        .filter_map(|(&(i, j), height)| {
            let lowest_adjacent = [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
                .iter()
                .map(|coord| input.get(coord).unwrap_or(&10))
                .reduce(|acc, adj_height| acc.min(adj_height))
                .unwrap();
            if height < lowest_adjacent {
                Some(height)
            } else {
                None
            }
        })
        .map(|&height| height as u32 + 1)
        .sum()
}

fn part2(input: &Cave) -> i32 {
    let mut basins: Vec<HashSet<(i32, i32)>> = vec![];
    let mut remaining: HashSet<(i32, i32)> = input
        .iter()
        .filter(|(_, &height)| height < 9)
        .map(|(coord, _)| coord.clone())
        .collect();
    while !remaining.is_empty() {
        let mut basin: HashSet<(i32, i32)> = HashSet::new();
        let start = remaining.iter().next().unwrap().clone();
        let mut stack: Vec<(i32, i32)> = vec![start];
        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            basin.insert((i, j));
            for coord in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
                if remaining.contains(&coord) {
                    remaining.remove(&coord);
                    stack.push(coord);
                }
            }
        }
        basins.push(basin);
    }
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    basins[..3]
        .iter()
        .map(|basin| basin.len() as i32)
        .fold(1, |acc, size| acc * size)
}
