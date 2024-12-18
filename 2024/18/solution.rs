use std::collections::{HashSet, VecDeque};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::process;

const GRID_SIZE: i32 = 70;

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vector(i32, i32);

type Input = Vec<Vector>;

impl Vector {
    const UP: Self = Self(0, -1);
    const DOWN: Self = Self(0, 1);
    const LEFT: Self = Self(-1, 0);
    const RIGHT: Self = Self(1, 0);
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut components = line.split(',').map(|n| n.parse().unwrap());
            Vector(components.next().unwrap(), components.next().unwrap())
        })
        .collect()
}

fn part1(input: &Input) -> i32 {
    let memory_space: HashSet<Vector> = input.iter().take(1024).copied().collect();
    let mut queue = VecDeque::from([(Vector(0, 0), 0)]);
    let mut visited = HashSet::new();
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == Vector(GRID_SIZE, GRID_SIZE) {
            return steps;
        }
        if pos.0 < 0 || GRID_SIZE < pos.0 || pos.1 < 0 || GRID_SIZE < pos.1 {
            continue;
        }
        if visited.contains(&pos) {
            continue;
        }
        if memory_space.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for direction in [Vector::DOWN, Vector::RIGHT, Vector::UP, Vector::LEFT] {
            queue.push_back((pos + direction, steps + 1));
        }
    }
    panic!("Path not found");
}

fn part2(input: &Input) -> String {
    let mut memory_space: HashSet<Vector> = input.iter().copied().collect();
    for byte in input.iter().rev() {
        memory_space.remove(byte);

        let mut stack = vec![Vector(0, 0)];
        let mut visited = HashSet::new();
        while let Some(pos) = stack.pop() {
            if pos == Vector(GRID_SIZE, GRID_SIZE) {
                return format!("{},{}", byte.0, byte.1);
            }
            if pos.0 < 0 || GRID_SIZE < pos.0 || pos.1 < 0 || GRID_SIZE < pos.1 {
                continue;
            }
            if visited.contains(&pos) {
                continue;
            }
            if memory_space.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            for direction in [Vector::DOWN, Vector::RIGHT, Vector::UP, Vector::LEFT] {
                stack.push(pos + direction);
            }
        }
    }
    panic!("Path not blocked");
}
