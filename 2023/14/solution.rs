use std::collections::BTreeSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Coord = [i64; 2];

#[derive(Debug, Clone)]
enum Rock {
    Rounded(Coord),
    Cube(Coord),
}

#[derive(Debug, Clone)]
struct Rocks {
    rounded: BTreeSet<Coord>,
    cube: BTreeSet<Coord>,
}

type Input = Rocks;

fn parse_input() -> Input {
    let (rounded, cube): (Vec<_>, Vec<_>) = fs::read_to_string("test.txt")
        .unwrap()
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(load, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                'O' => Some(Rock::Rounded([x as i64, load as i64 + 1])),
                '#' => Some(Rock::Cube([x as i64, load as i64 + 1])),
                _ => None,
            })
        })
        .partition(|rock| match rock {
            Rock::Rounded(_) => true,
            Rock::Cube(_) => false,
        });
    Rocks {
        rounded: rounded
            .into_iter()
            .map(|rock| match rock {
                Rock::Rounded(coord) => coord,
                _ => panic!(),
            })
            .collect(),
        cube: cube
            .into_iter()
            .map(|rock| match rock {
                Rock::Rounded(coord) => coord,
                _ => panic!(),
            })
            .collect(),
    }
}

fn part1(input: &Input) -> i64 {
    input.cube.iter();
    0
}

fn part2(input: &Input) -> i64 {
    0
}
