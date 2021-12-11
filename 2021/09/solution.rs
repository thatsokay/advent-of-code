use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Vec<i32>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, height)| {
                let left = row.get((j as i32 - 1) as usize).or(Some(&10));
                let right = row.get((j as i32 + 1) as usize).or(Some(&10));
                let up = input
                    .get((i as i32 - 1) as usize)
                    .and_then(|row| row.get(j))
                    .or(Some(&10));
                let down = input
                    .get((i as i32 + 1) as usize)
                    .and_then(|row| row.get(j))
                    .or(Some(&10));
                match (left, right, up, down) {
                    (Some(a), Some(b), Some(c), Some(d))
                        if (height < a && height < b && height < c && height < d) =>
                    {
                        Some(height)
                    }
                    _ => None,
                }
            })
        })
        .map(|height| height + 1)
        .sum()
}

fn part2(input: &[Vec<i32>]) -> i32 {
    0
}
