use std::collections::HashMap;
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

struct WordSearch(HashMap<Vector, char>);

struct ReadWord<'a> {
    word_search: &'a WordSearch,
    start: Vector,
    step: Vector,
    index: i32,
}

type Input = WordSearch;

impl Vector {
    fn add(&self, dv: &Self) -> Self {
        Vector(self.0 + dv.0, self.1 + dv.1)
    }

    fn scale(&self, factor: i32) -> Self {
        Vector(self.0 * factor, self.1 * factor)
    }
}

impl<'a> WordSearch {
    fn read_word(&'a self, start: Vector, step: Vector) -> ReadWord<'a> {
        ReadWord {
            word_search: self,
            start,
            step,
            index: 0,
        }
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
    WordSearch(
        fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Vector(x as i32, y as i32), c))
            })
            .collect(),
    )
}

fn part1(input: &Input) -> usize {
    input
        .0
        .iter()
        .filter(|&(_coord, &c)| c == 'X')
        .flat_map(|(&coord, _)| {
            (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| Vector(x, y)))
                .filter(|v| v.0 != 0 || v.1 != 0)
                .map(move |step| input.read_word(coord, step).take(4).collect::<String>())
        })
        .filter(|word| word == "XMAS")
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .0
        .iter()
        .filter(|&(_coord, &c)| c == 'A')
        .filter(|(&coord, _)| {
            [Vector(-1, -1), Vector(-1, 1)]
                .iter()
                .map(move |step| {
                    input
                        .read_word(coord.add(step), step.scale(-1))
                        .take(3)
                        .collect::<String>()
                })
                .all(|word| word == "MAS" || word == "SAM")
        })
        .count()
}
