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

#[derive(Debug, Clone)]
struct Cache {
    cache: HashMap<(u64, u8), u64>,
}

impl Cache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn expand_stone(&mut self, stone: u64, times: u8) -> u64 {
        if times == 0 {
            return 1;
        }
        if let Some(&value) = self.cache.get(&(stone, times)) {
            return value;
        }
        if stone == 0 {
            let result = self.expand_stone(1, times - 1);
            self.cache.insert((0, times), result);
            return result;
        }
        let digit_count = stone.ilog10() + 1;
        if digit_count % 2 == 0 {
            let split = 10_u64.pow(digit_count / 2);
            let result = self.expand_stone(stone / split, times - 1)
                + self.expand_stone(stone % split, times - 1);
            self.cache.insert((stone, times), result);
            return result;
        }
        let result = self.expand_stone(stone * 2024, times - 1);
        self.cache.insert((stone, times), result);
        return result;
    }
}

type Input = Vec<u64>;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &Input) -> u64 {
    let mut cache = Cache::new();
    input
        .iter()
        .map(|&stone| cache.expand_stone(stone, 25))
        .sum()
}

fn part2(input: &Input) -> u64 {
    let mut cache = Cache::new();
    input
        .iter()
        .map(|&stone| cache.expand_stone(stone, 75))
        .sum()
}
