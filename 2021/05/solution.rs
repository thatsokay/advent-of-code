use std::cmp::max;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

type Vent = (Point, Point);

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Vent> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();
            (
                Point {
                    x: x1.parse().unwrap(),
                    y: y1.parse().unwrap(),
                },
                Point {
                    x: x2.parse().unwrap(),
                    y: y2.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn part1(input: &[Vent]) -> u32 {
    let non_diagonal_vents: Vec<_> = input
        .iter()
        .filter(|vent| vent.0.x == vent.1.x || vent.0.y == vent.1.y)
        .map(|vent| vent.clone())
        .collect();
    count_overlaps(&non_diagonal_vents)
}

fn part2(input: &[Vent]) -> u32 {
    count_overlaps(input)
}

fn count_overlaps(vents: &[Vent]) -> u32 {
    let mut result = HashMap::new();
    for vent in vents {
        let x_diff = vent.1.x - vent.0.x;
        let y_diff = vent.1.y - vent.0.y;
        let max_diff = max(x_diff.abs(), y_diff.abs());
        let x_step = x_diff / max_diff;
        let y_step = y_diff / max_diff;
        for i in 0..=max_diff {
            *result
                .entry(Point {
                    x: vent.0.x + x_step * i,
                    y: vent.0.y + y_step * i,
                })
                .or_insert(0) += 1;
        }
    }
    result.values().filter(|&&count| count > 1).count() as u32
}
