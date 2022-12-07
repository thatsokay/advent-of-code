use std::collections::HashMap;
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
    let directory_sizes = navigate_filesystem(input);
    directory_sizes
        .values()
        .filter(|&&size| size <= 100000)
        .sum()
}

fn part2(input: &[String]) -> u32 {
    let directory_sizes = navigate_filesystem(input);
    let required_size =
        directory_sizes.get(&vec![String::from("/")]).unwrap() + 30000000 - 70000000;
    *directory_sizes
        .values()
        .filter(|&&size| size >= required_size)
        .min()
        .unwrap()
}

fn navigate_filesystem(lines: &[String]) -> HashMap<Vec<String>, u32> {
    let mut directory_path = vec![String::from("/")];
    let mut directory_sizes = HashMap::new();
    lines.iter().for_each(|line| match &line[0..4] {
        "$ cd" => match &line[5..] {
            "/" => {
                directory_path.drain(1..);
            }
            ".." => {
                directory_path.pop();
            }
            _ => directory_path.push(line[5..].to_string()),
        },
        "$ ls" => {}
        "dir " => {}
        _ => (0..(directory_path.len())).for_each(|i| {
            let file_size = line.split_once(' ').unwrap().0.parse().unwrap();
            let path = directory_path[0..(i + 1)].to_vec();
            directory_sizes
                .entry(path)
                .and_modify(|size| *size += file_size)
                .or_insert(file_size);
        }),
    });
    directory_sizes
}
