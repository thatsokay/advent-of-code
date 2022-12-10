use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<(char, u8)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (direction, steps) = line.split_once(' ').unwrap();
            (direction.chars().next().unwrap(), steps.parse().unwrap())
        })
        .collect()
}

fn part1(input: &[(char, u8)]) -> usize {
    move_rope(2, input)
}

fn part2(input: &[(char, u8)]) -> usize {
    move_rope(10, input)
}

fn move_rope(length: usize, moves: &[(char, u8)]) -> usize {
    let mut knots: Vec<[i32; 2]> = vec![[0, 0]; length];
    let mut visited = HashSet::new();
    for &(direction, steps) in moves {
        for _ in 0..steps {
            match direction {
                'L' => {
                    knots[0][0] -= 1;
                }
                'R' => {
                    knots[0][0] += 1;
                }
                'U' => {
                    knots[0][1] += 1;
                }
                'D' => {
                    knots[0][1] -= 1;
                }
                _ => unreachable!(),
            }
            for i in 0..(length - 1) {
                match [knots[i][0] - knots[i + 1][0], knots[i][1] - knots[i + 1][1]] {
                    [x, y] if x.abs() == 2 => {
                        knots[i + 1][0] += x / 2;
                        if y != 0 {
                            knots[i + 1][1] += y / y.abs();
                        }
                    }
                    [x, y] if y.abs() == 2 => {
                        knots[i + 1][1] += y / 2;
                        if x != 0 {
                            knots[i + 1][0] += x / x.abs();
                        }
                    }
                    _ => {}
                }
            }
            visited.insert(knots[length - 1].clone());
        }
    }
    visited.len()
}
