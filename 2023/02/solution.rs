use std::cmp::max;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    pulls: Vec<HashMap<String, u32>>,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Game> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (game_label, pulls_description) = line.split_once(": ").unwrap();
            let id = game_label
                .split_once(' ')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap();
            let pulls = pulls_description
                .split("; ")
                .map(|pull_description| {
                    let mut pulls = HashMap::new();
                    for cube_description in pull_description.split(", ") {
                        let (count_description, colour) = cube_description.split_once(' ').unwrap();
                        let count = count_description.parse::<u32>().unwrap();
                        pulls
                            .entry(colour.to_string())
                            .and_modify(|x| *x += count)
                            .or_insert(count);
                    }
                    pulls
                })
                .collect();
            Game { id, pulls }
        })
        .collect()
}

fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.pulls.iter().all(|pull| {
                pull.get("red").unwrap_or(&0) <= &12
                    && pull.get("green").unwrap_or(&0) <= &13
                    && pull.get("blue").unwrap_or(&0) <= &14
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            game.pulls
                .iter()
                .flatten()
                .fold([0, 0, 0], |[r, g, b], (colour, &count)| match &colour[..] {
                    "red" => [max(r, count), g, b],
                    "green" => [r, max(g, count), b],
                    "blue" => [r, g, max(b, count)],
                    _ => panic!(),
                })
                .iter()
                .product::<u32>()
        })
        .sum()
}
