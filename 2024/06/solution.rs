use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector(i32, i32);

struct Map(HashMap<Vector, char>);

struct ReadWord<'a> {
    word_search: &'a Map,
    start: Vector,
    step: Vector,
    index: i32,
}

type Input = Map;

impl Vector {
    fn add(&self, dv: &Self) -> Self {
        Vector(self.0 + dv.0, self.1 + dv.1)
    }

    fn scale(&self, factor: i32) -> Self {
        Vector(self.0 * factor, self.1 * factor)
    }

    fn rotate_clockwise(&self) -> Self {
        Vector(-self.1, self.0)
    }
}

impl Iterator for ReadWord<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let coord = self.start.add(&self.step.scale(self.index));
        self.index += 1;
        self.word_search.0.get(&coord).copied()
    }
}

fn parse_input(file_path: OsString) -> Input {
    Map(fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Vector(x as i32, y as i32), c))
        })
        .collect())
}

fn part1(input: &Input) -> usize {
    let mut guard_coord = *input.0.iter().find(|(_, &c)| c == '^').unwrap().0;
    let mut guard_direction = Vector(0, -1);
    let mut visited = HashSet::from([guard_coord]);
    loop {
        let next_coord = guard_coord.add(&guard_direction);
        match input.0.get(&next_coord) {
            None => break,
            Some('#') => {
                guard_direction = guard_direction.rotate_clockwise();
            }
            Some(_) => {
                guard_coord = next_coord;
                visited.insert(next_coord);
            }
        }
    }
    visited.len()
}

fn part2(input: &Input) -> usize {
    let guard_coord = *input.0.iter().find(|(_, &c)| c == '^').unwrap().0;
    input
        .0
        .iter()
        .filter(|(_, &c)| c == '.')
        .map(|entry| entry.0)
        .filter(|&&obstruction| {
            println!("{:?}", obstruction);
            let mut guard_coord = guard_coord.clone();
            let mut guard_direction = Vector(0, -1);
            let mut visited = HashSet::from([(guard_coord, guard_direction)]);
            loop {
                let next_coord = guard_coord.add(&guard_direction);
                match input.0.get(&next_coord) {
                    None => return false,
                    Some('#') => {
                        guard_direction = guard_direction.rotate_clockwise();
                    }
                    Some(_) => {
                        if next_coord == obstruction {
                            guard_direction = guard_direction.rotate_clockwise();
                            continue;
                        }
                        if visited.contains(&(next_coord, guard_direction)) {
                            return true;
                        } else {
                            guard_coord = next_coord;
                            visited.insert((next_coord, guard_direction));
                        }
                    }
                }
            }
        })
        .count()
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, String> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    match get_first_arg() {
        Ok(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
