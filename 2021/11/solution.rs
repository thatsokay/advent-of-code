use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Octopus {
    Unflashed(u8),
    Flashed,
}

type Grid = HashMap<(isize, isize), Octopus>;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Grid {
    let mut result: Grid = HashMap::new();
    for (i, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            result.insert(
                (i as isize, j as isize),
                Octopus::Unflashed(c.to_digit(10).unwrap() as u8),
            );
        }
    }
    result
}

fn part1(input: &Grid) -> u32 {
    let grid = input.clone();
    (0..100).scan(grid, |acc, _| Some(count_flashes(acc))).sum()
}

fn part2(input: &Grid) -> u32 {
    let grid = input.clone();
    (1..)
        .scan(grid, |acc, i| Some((i, count_flashes(acc))))
        .find(|&(_, flashes)| flashes == 100)
        .unwrap()
        .0 as u32
}

fn count_flashes(grid: &mut Grid) -> u32 {
    for octopus in grid.values_mut() {
        match octopus {
            Octopus::Unflashed(x) => *octopus = Octopus::Unflashed(*x + 1),
            _ => unreachable!(),
        }
    }
    let mut count: u32 = 0;
    loop {
        let flashing: Vec<_> = grid
            .iter()
            .filter_map(|(&(i, j), octopus)| match octopus {
                Octopus::Unflashed(x) if *x > 9 => Some((i, j)),
                _ => None,
            })
            .collect();
        if flashing.len() == 0 {
            break;
        }
        count += flashing.len() as u32;
        for (flash_i, flash_j) in flashing {
            grid.insert((flash_i, flash_j), Octopus::Flashed);
            for i in (flash_i - 1)..=(flash_i + 1) {
                for j in (flash_j - 1)..=(flash_j + 1) {
                    grid.entry((i, j)).and_modify(|octopus| {
                        if let Octopus::Unflashed(x) = octopus {
                            *octopus = Octopus::Unflashed(*x + 1);
                        }
                    });
                }
            }
        }
    }
    for octopus in grid.values_mut() {
        match octopus {
            Octopus::Flashed => *octopus = Octopus::Unflashed(0),
            _ => {}
        }
    }
    count
}
