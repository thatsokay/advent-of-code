use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first_digit * 10 + last_digit
        })
        .sum()
}

fn part2(input: &[String]) -> u32 {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .iter()
        .map(|line| {
            let first_digit = (0..(line.len()))
                .find_map(|start_index| {
                    let subline = &line[start_index..];
                    words
                        .iter()
                        .enumerate()
                        .find(|(_i, &word)| subline.starts_with(word))
                        .and_then(|(i, _word)| Some(i as u32))
                        .or_else(|| subline.chars().next().unwrap().to_digit(10))
                })
                .unwrap();
            let last_digit = (1..(line.len() + 1))
                .rev()
                .find_map(|end_index| {
                    let subline = &line[..end_index];
                    words
                        .iter()
                        .enumerate()
                        .find(|(_i, &word)| subline.ends_with(word))
                        .and_then(|(i, _word)| Some(i as u32))
                        .or_else(|| subline.chars().rev().next().unwrap().to_digit(10))
                })
                .unwrap();
            first_digit * 10 + last_digit
        })
        .sum()
}
