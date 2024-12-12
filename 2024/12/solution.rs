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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Plot {
    plant: char,
    coord: Vector,
}

#[derive(Debug, Clone)]
struct RegionChunk {
    plant: char,
    start_x: i32,
    end_x: i32,
}

#[derive(Debug, Clone)]
struct OpenRegion {
    plant: char,
    open_chunks: Vec<RegionChunk>,
    perimeter: u32,
    area: u32,
}

type Input = Vec<Vec<Plot>>;

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
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| Plot {
                    plant: c,
                    coord: Vector(x as i32, y as i32),
                })
                .collect()
        })
        .collect()
}

fn part1(input: &Input) -> u32 {
    let open_regions: Vec<OpenRegion> = vec![];
    for row in input.iter() {
        let chunks = row.chunk_by(|a, b| a.plant == b.plant).map(|chunk| {
            RegionChunk {
                plant: chunk[0].plant,
                start_x: chunk[0].coord.0,
                end_x: chunk[chunk.len() - 1].coord.0,
            };
        });
        for chunk in chunks {

        }
    }
    0
}

fn part2(input: &Input) -> usize {
    0
}
