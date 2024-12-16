use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vector(i32, i32);

type Input = HashMap<Vector, char>;

impl Vector {
    const RIGHT: Self = Self(1, 0);
}

impl Vector {
    fn rotate_clockwise(self) -> Self {
        Self(-self.1, self.0)
    }

    fn rotate_anticlockwise(self) -> Self {
        Self(self.1, -self.0)
    }
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
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (Vector(x as i32, y as i32), c))
        })
        .collect()
}

fn part1(input: &Input) -> u32 {
    let &end_coord = input.iter().find(|(_, c)| **c == 'E').unwrap().0;
    let &start_coord = input.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0, start_coord, Vector::RIGHT)]);
    let mut visited = HashSet::new();
    while let Some((Reverse(score), steps, coord, direction)) = heap.pop() {
        if coord == end_coord {
            return score;
        }
        if visited.contains(&(coord, direction)) {
            continue;
        }
        if let Some('#') = input.get(&coord) {
            continue;
        }
        visited.insert((coord, direction));
        heap.push((Reverse(score + 1), steps + 1, coord + direction, direction));
        heap.push((
            Reverse(score + 1000),
            steps,
            coord,
            direction.rotate_clockwise(),
        ));
        heap.push((
            Reverse(score + 1000),
            steps,
            coord,
            direction.rotate_anticlockwise(),
        ));
    }
    panic!("Path not found");
}

fn part2(input: &Input) -> usize {
    let &end_coord = input.iter().find(|(_, c)| **c == 'E').unwrap().0;
    let &start_coord = input.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0, vec![start_coord], Vector::RIGHT)]);
    let mut visited = HashMap::<(Vector, Vector), u32>::new();
    let mut seats = HashSet::new();
    let mut min_score = u32::MAX;
    while let Some((Reverse(score), steps, path, direction)) = heap.pop() {
        let &coord = path.last().unwrap();
        if score > min_score {
            continue;
        }
        if coord == end_coord {
            min_score = score;
            for path_coord in path {
                seats.insert(path_coord);
            }
            continue;
        }
        if let Some(&prev_score) = visited.get(&(coord, direction)) {
            if prev_score < score {
                continue;
            }
        }
        if let Some('#') = input.get(&coord) {
            continue;
        }
        visited.insert((coord, direction), score);
        heap.push((
            Reverse(score + 1),
            steps + 1,
            [path.clone(), vec![coord + direction]].concat(),
            direction,
        ));
        heap.push((
            Reverse(score + 1000),
            steps,
            path.clone(),
            direction.rotate_clockwise(),
        ));
        heap.push((
            Reverse(score + 1000),
            steps,
            path,
            direction.rotate_anticlockwise(),
        ));
    }
    seats.len()
}
