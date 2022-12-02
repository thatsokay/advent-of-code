use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<[char; 2]> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent_play = chars.next().unwrap();
            chars.next();
            let my_play = chars.next().unwrap();
            [opponent_play, my_play]
        })
        .collect()
}

fn part1(input: &[[char; 2]]) -> i32 {
    input
        .iter()
        .map(|[opponent_play, my_play]| match my_play {
            'X' => match opponent_play {
                'A' => 4,
                'B' => 1,
                'C' => 7,
                _ => panic!(),
            },
            'Y' => match opponent_play {
                'A' => 8,
                'B' => 5,
                'C' => 2,
                _ => panic!(),
            },
            'Z' => match opponent_play {
                'A' => 3,
                'B' => 9,
                'C' => 6,
                _ => panic!(),
            },
            _ => panic!(),
        })
        .sum()
}

fn part2(input: &[[char; 2]]) -> i32 {
    input
        .iter()
        .map(|[opponent_play, my_play]| match my_play {
            'X' => match opponent_play {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => panic!(),
            },
            'Y' => match opponent_play {
                'A' => 4,
                'B' => 5,
                'C' => 6,
                _ => panic!(),
            },
            'Z' => match opponent_play {
                'A' => 8,
                'B' => 9,
                'C' => 7,
                _ => panic!(),
            },
            _ => panic!(),
        })
        .sum()
}
