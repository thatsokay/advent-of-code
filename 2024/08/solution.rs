use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

static MAP_SIZE: i32 = 50;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector(i32, i32);

type Input = HashMap<char, Vec<Vector>>;

impl Vector {
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    fn subtract(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }

    fn scale(&self, factor: i32) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }

    fn is_in_map_bounds(&self) -> bool {
        0 <= self.0 && self.0 < MAP_SIZE && 0 <= self.1 && self.1 < MAP_SIZE
    }

    fn normalise(&self) -> Self {
        let mut a = self.0;
        let mut b = self.1;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        Self(self.0 / a, self.1 / a)
    }
}

fn parse_input(file_path: OsString) -> Input {
    let mut antennas: Vec<_> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (c, Vector(x as i32, y as i32)))
        })
        .collect();
    antennas.sort_by_key(|&(c, _)| c);
    antennas
        .chunk_by(|(c, _), (d, _)| c == d)
        .map(|chunk| {
            let (left, right): (Vec<_>, Vec<_>) = chunk.iter().cloned().unzip();
            (left[0], right)
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|(_, antennas)| {
            (0..antennas.len())
                .flat_map(move |j| (0..j).map(move |i| [antennas[i], antennas[j]]))
                .flat_map(|[a, b]| {
                    let difference = b.subtract(&a);
                    [a.subtract(&difference), b.add(&difference)]
                })
                .filter(|v| v.is_in_map_bounds())
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|(_, antennas)| {
            (0..antennas.len())
                .flat_map(move |j| (0..j).map(move |i| [antennas[i], antennas[j]]))
                .flat_map(|[a, b]| {
                    let step = b.subtract(&a).normalise();
                    (0..)
                        .map(move |i| a.add(&step.scale(i)))
                        .take_while(|v| v.is_in_map_bounds())
                        .chain(
                            (1..)
                                .map(move |i| a.subtract(&step.scale(i)))
                                .take_while(|v| v.is_in_map_bounds()),
                        )
                })
        })
        .collect::<HashSet<_>>()
        .len()
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
