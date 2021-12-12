use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: &[Vec<char>]) -> u32 {
    input
        .iter()
        .filter_map(|chars| {
            find_corrupted(chars).and_then(|unmatching| match unmatching {
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                '>' => Some(25137),
                _ => panic!(),
            })
        })
        .sum()
}

fn part2(input: &[Vec<char>]) -> u64 {
    let mut incomplete_scores: Vec<_> = input
        .iter()
        .filter_map(|chars| find_incomplete(chars))
        .map(|remaining| {
            remaining
                .into_iter()
                .rev()
                .map(|c| -> u64 {
                    match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(),
                    }
                })
                .fold(0, |acc, points| acc * 5 + points)
        })
        .collect();
    incomplete_scores.sort();
    incomplete_scores[incomplete_scores.len() / 2]
}

fn find_corrupted(chars: &[char]) -> Option<char> {
    let mut chunk_stack: Vec<char> = vec![];
    for &c in chars {
        if "([{<".contains(c) {
            chunk_stack.push(c);
            continue;
        }
        match (chunk_stack.pop(), c) {
            (Some('('), ')') => {}
            (Some('['), ']') => {}
            (Some('{'), '}') => {}
            (Some('<'), '>') => {}
            _ => return Some(c),
        }
    }
    None
}

fn find_incomplete(chars: &[char]) -> Option<Vec<char>> {
    let mut chunk_stack: Vec<char> = vec![];
    for &c in chars {
        if "([{<".contains(c) {
            chunk_stack.push(c);
            continue;
        }
        match (chunk_stack.pop(), c) {
            (Some('('), ')') => {}
            (Some('['), ']') => {}
            (Some('{'), '}') => {}
            (Some('<'), '>') => {}
            _ => return None,
        }
    }
    Some(chunk_stack)
}
