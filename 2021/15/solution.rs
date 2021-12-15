use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

type RiskMap = HashMap<[i16; 2], u32>;

struct Cavern {
    size: usize,
    risks: RiskMap,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Cavern {
    let mut risks: RiskMap = HashMap::new();
    let input = fs::read_to_string("input.txt").unwrap();
    let size = input.lines().next().unwrap().len();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            risks.insert([i as i16, j as i16], c.to_digit(10).unwrap() as u32);
        }
    }
    Cavern { size, risks }
}

fn part1(input: &Cavern) -> u32 {
    lowest_risk(input)
}

fn part2(input: &Cavern) -> u32 {
    let cavern = full_cavern(input);
    lowest_risk(&cavern)
}

fn lowest_risk(cavern: &Cavern) -> u32 {
    let mut total_risks: RiskMap = HashMap::from([([0, 0], 0)]);
    let mut heap = BinaryHeap::from([Reverse((0, [0, 0]))]);
    while let Some(Reverse((current_risk, [i, j]))) = heap.pop() {
        if i == j && i == cavern.size as i16 - 1 {
            return current_risk;
        }
        for coord in [[i, j - 1], [i, j + 1], [i - 1, j], [i + 1, j]] {
            if let Some(coord_risk) = cavern.risks.get(&coord) {
                match total_risks.get(&coord) {
                    Some(total_coord_risk) if current_risk + coord_risk < *total_coord_risk => {
                        total_risks.insert(coord, current_risk + coord_risk);
                        heap.push(Reverse((current_risk + coord_risk, coord)));
                    }
                    None => {
                        total_risks.insert(coord, current_risk + coord_risk);
                        heap.push(Reverse((current_risk + coord_risk, coord)));
                    }
                    _ => {}
                }
            }
        }
    }
    unreachable!();
}

fn full_cavern(cavern: &Cavern) -> Cavern {
    let size = cavern.size * 5;
    let mut risks = cavern.risks.clone();
    for right in 0..5 {
        for down in 0..5 {
            if right == 0 && down == 0 {
                continue;
            }
            for (&[i, j], &risk) in cavern.risks.iter() {
                risks.insert(
                    [
                        i + down * cavern.size as i16,
                        j + right * cavern.size as i16,
                    ],
                    (risk - 1 + right as u32 + down as u32) % 9 + 1,
                );
            }
        }
    }
    Cavern { size, risks }
}
