use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::ops::{Add, Div, Mul, Neg, Sub};
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

#[derive(Debug, Clone)]
struct Map {
    value: HashMap<Vector, char>,
}

#[derive(Debug, Clone)]
struct Robot {
    map: Map,
    movements: Vec<char>,
}

type Input = Robot;

impl Vector {
    const UP: Self = Self(0, -1);
    const DOWN: Self = Self(0, 1);
    const LEFT: Self = Self(-1, 0);
    const RIGHT: Self = Self(1, 0);
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, factor: i32) -> Self::Output {
        Self(self.0 * factor, self.1 * factor)
    }
}

impl Map {
    fn from(value: HashMap<Vector, char>) -> Self {
        Self { value }
    }

    fn move_piece(&mut self, coord: Vector, direction: Vector) -> Option<Vector> {
        match self.value.get(&coord) {
            None => Some(coord),
            Some('#') => None,
            _ => self
                .move_piece(coord + direction, direction)
                .map(|move_to| {
                    let piece = self.value.remove(&coord).unwrap();
                    self.value.insert(move_to, piece);
                    coord
                }),
        }
    }
}

fn parse_input(file_path: OsString) -> Input {
    let content = fs::read_to_string(file_path).unwrap();
    let (map_content, movements_content) = content.split_once("\n\n").unwrap();
    let map = Map::from(
        map_content
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, c)| (Vector(x as i32, y as i32), c))
            })
            .collect(),
    );
    let movements = movements_content
        .lines()
        .flat_map(|line| line.chars())
        .collect();
    Robot { map, movements }
}

fn part1(input: &Input) -> i32 {
    let Robot { mut map, movements } = input.clone();
    let mut robot_coord = *map.value.iter().find(|(_, c)| **c == '@').unwrap().0;
    for movement in movements {
        let direction = match movement {
            '^' => Vector::UP,
            'v' => Vector::DOWN,
            '<' => Vector::LEFT,
            '>' => Vector::RIGHT,
            _ => panic!("Unrecognised movement"),
        };
        if let Some(moved_to_coord) = map.move_piece(robot_coord, direction) {
            robot_coord = moved_to_coord;
        }
    }
    map.value
        .iter()
        .filter(|(_, c)| **c == 'O')
        .map(|(coord, _)| coord.0 + coord.1 * 100)
        .sum()
}

fn part2(input: &Input) -> usize {
    0
}
