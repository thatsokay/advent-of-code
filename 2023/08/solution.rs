use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

enum Direction {
    Left,
    Right,
}

struct Node {
    left: String,
    right: String,
}

struct Map {
    instructions: Vec<Direction>,
    network: HashMap<String, Node>,
}

type Input = Map;

fn parse_input() -> Input {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let (instructions_line, network_lines) = file_content.split_once("\n\n").unwrap();
    let instructions = instructions_line
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        })
        .collect();
    let network = network_lines
        .lines()
        .map(|line| {
            let (id, neighbours) = line.split_once(" = ").unwrap();
            let mut neighbour_ids = neighbours[1..(neighbours.len() - 1)]
                .split(", ")
                .map(|neighbour_id| neighbour_id.to_string());
            (
                id.to_string(),
                Node {
                    left: neighbour_ids.next().unwrap(),
                    right: neighbour_ids.next().unwrap(),
                },
            )
        })
        .collect();
    Map {
        instructions,
        network,
    }
}

fn part1(input: &Input) -> u32 {
    input
        .instructions
        .iter()
        .cycle()
        .scan("AAA", |node_id, instruction| {
            let node = &input.network[&node_id[..]];
            let next_node_id = match instruction {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };
            *node_id = next_node_id;
            if next_node_id == "ZZZ" {
                None
            } else {
                Some(next_node_id)
            }
        })
        .count() as u32
        + 1
}

fn part2(input: &Input) -> u32 {
    0
}
