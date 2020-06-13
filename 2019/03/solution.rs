use std::collections::{HashMap, HashSet};
use std::fs;

type Coordinate = (i32, i32);
type Wire = HashMap<Coordinate, i32>;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (Wire, Wire) {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut wires = input
        .split("\n")
        .map(|line| {
            line.split(",").map(|segment| {
                let (direction, distance) = segment.split_at(1);
                (direction.to_string(), distance.parse::<i32>().unwrap())
            })
        })
        .map(|wire| trace_wire(wire));
    (wires.next().unwrap(), wires.next().unwrap())
}

fn part1((wire1, wire2): &(Wire, Wire)) -> i32 {
    let keys1: HashSet<(i32, i32)> = wire1.keys().cloned().collect();
    let keys2: HashSet<(i32, i32)> = wire2.keys().cloned().collect();
    keys1
        .intersection(&keys2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part2((wire1, wire2): &(Wire, Wire)) -> i32 {
    let keys1: HashSet<(i32, i32)> = wire1.keys().cloned().collect();
    let keys2: HashSet<(i32, i32)> = wire2.keys().cloned().collect();
    keys1
        .intersection(&keys2)
        .map(|coordinate| wire1[coordinate] + wire2[coordinate])
        .min()
        .unwrap()
}

fn trace_wire<T: Iterator<Item = (String, i32)>>(wire: T) -> Wire {
    let mut visited = HashMap::new();
    let mut position = (0, 0);
    let mut steps = 0;
    for (direction, distance) in wire {
        for _ in 0..distance {
            match &direction[..] {
                "U" => position.1 += 1,
                "D" => position.1 -= 1,
                "L" => position.0 -= 1,
                "R" => position.0 += 1,
                _ => (),
            }
            steps += 1;
            visited.insert(position, steps);
        }
    }
    return visited;
}
