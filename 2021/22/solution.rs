use std::collections::HashSet;
use std::fs;

type ReactorCore = HashSet<[i32; 3]>;

#[derive(Clone, Debug)]
struct RebootStep {
    on: bool,
    x: [i32; 2],
    y: [i32; 2],
    z: [i32; 2],
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<RebootStep> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let on_line = line.strip_prefix("on ");
            let off_line = line.strip_prefix("off ");
            let on = on_line.is_some();
            let ranges_line = on_line.or(off_line).unwrap();
            let ranges: Vec<_> = ranges_line
                .split(',')
                .map(|range_line| {
                    let range = range_line[2..].split_once("..").unwrap();
                    [range.0.parse().unwrap(), range.1.parse().unwrap()]
                })
                .collect();
            RebootStep {
                on,
                x: ranges[0],
                y: ranges[1],
                z: ranges[2],
            }
        })
        .collect()
}

fn part1(input: &[RebootStep]) -> u32 {
    let mut reactor: ReactorCore = HashSet::new();
    for step in input {
        for x in step.x[0].max(-50)..=step.x[1].min(50) {
            for y in step.y[0].max(-50)..=step.y[1].min(50) {
                for z in step.z[0].max(-50)..=step.z[1].min(50) {
                    if step.on {
                        reactor.insert([x, y, z]);
                    } else {
                        reactor.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
    reactor.len() as u32
}

fn part2(input: &[RebootStep]) -> u32 {
    0
}
