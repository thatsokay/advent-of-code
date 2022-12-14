use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Blockage {
    Rock,
    Sand,
}

use Blockage::*;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> HashMap<[i32; 2], Blockage> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let mut coord_split = coord.split(',').map(|i| i.parse::<i32>().unwrap());
                    [coord_split.next().unwrap(), coord_split.next().unwrap()]
                })
                .collect::<Vec<_>>()
                .windows(2)
                .flat_map(|window| {
                    let mut x_range = [window[0][0], window[1][0]];
                    x_range.sort();
                    let mut y_range = [window[0][1], window[1][1]];
                    y_range.sort();
                    (x_range[0]..=x_range[1])
                        .flat_map(move |x| (y_range[0]..=y_range[1]).map(move |y| ([x, y], Rock)))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(input: &HashMap<[i32; 2], Blockage>) -> usize {
    let cave = fill_sand(input, false);
    cave.values().filter(|&&blockage| blockage == Sand).count()
}

fn part2(input: &HashMap<[i32; 2], Blockage>) -> usize {
    let cave = fill_sand(input, true);
    cave.values().filter(|&&blockage| blockage == Sand).count()
}

fn fill_sand(rocks: &HashMap<[i32; 2], Blockage>, has_floor: bool) -> HashMap<[i32; 2], Blockage> {
    let mut cave = rocks.clone();
    let rocks_max_y = cave.keys().map(|[_, y]| *y).max().unwrap();
    loop {
        let mut sand_coord = [500, 0];
        loop {
            let end_condition = if has_floor {
                cave.contains_key(&[500, 0])
            } else {
                sand_coord[1] >= rocks_max_y
            };
            if end_condition {
                return cave;
            }
            match [[0, 1], [-1, 1], [1, 1]]
                .iter()
                .map(|[x, y]| [sand_coord[0] + x, sand_coord[1] + y])
                .find(|coord| {
                    !cave.contains_key(coord) && (!has_floor || coord[1] < rocks_max_y + 2)
                }) {
                Some([x, y]) => {
                    sand_coord[0] = x;
                    sand_coord[1] = y;
                }
                None => {
                    cave.insert(sand_coord, Sand);
                    break;
                }
            }
        }
    }
}
