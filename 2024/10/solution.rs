use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector(i32, i32);

type Input = HashMap<Vector, u8>;

impl Vector {
    const UP: Self = Self(0, -1);
    const DOWN: Self = Self(0, 1);
    const LEFT: Self = Self(-1, 0);
    const RIGHT: Self = Self(1, 0);

    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Vector(x as i32, y as i32), c.to_digit(10).unwrap() as u8))
        })
        .collect()
}

fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(trailhead, _)| {
            let mut score = 0;
            let mut queue = vec![*trailhead];
            let mut visited = HashSet::new();
            while let Some(coord) = queue.pop() {
                visited.insert(coord);
                let height = input[&coord];
                if height == 9 {
                    score += 1;
                    continue;
                }
                for direction in [Vector::UP, Vector::DOWN, Vector::LEFT, Vector::RIGHT] {
                    let adjacent_coord = coord.add(&direction);
                    if !visited.contains(&adjacent_coord) {
                        if let Some(&adjacent_height) = input.get(&adjacent_coord) {
                            if adjacent_height == height + 1 {
                                queue.push(adjacent_coord);
                            }
                        }
                    }
                }
            }
            score
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(trailhead, _)| {
            let mut score = 0;
            let mut queue = vec![*trailhead];
            while let Some(coord) = queue.pop() {
                let height = input[&coord];
                if height == 9 {
                    score += 1;
                    continue;
                }
                for direction in [Vector::UP, Vector::DOWN, Vector::LEFT, Vector::RIGHT] {
                    let adjacent_coord = coord.add(&direction);
                    if let Some(&adjacent_height) = input.get(&adjacent_coord) {
                        if adjacent_height == height + 1 {
                            queue.push(adjacent_coord);
                        }
                    }
                }
            }
            score
        })
        .sum()
}
