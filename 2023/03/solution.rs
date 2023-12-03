use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Coord = [i32; 2];

#[derive(Debug, Clone)]
enum Token {
    Number { coord: Coord, value: u32 },
    Symbol { coord: Coord, value: char },
}

#[derive(Debug, Clone)]
struct Map {
    numbers: HashMap<Coord, u32>,
    symbols: HashMap<Coord, char>,
}

type Input = Map;

fn parse_input() -> Input {
    let tokens: Vec<Token> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .chain(['.'])
                .enumerate()
                .fold((vec![], None), |acc, (x, c)| match c {
                    '.' => match acc {
                        (tokens, Some(n)) => (
                            [
                                &tokens[..],
                                &[Token::Number {
                                    coord: [x as i32 - 1, y as i32],
                                    value: n,
                                }],
                            ]
                            .concat(),
                            None,
                        ),
                        _ => acc,
                    },
                    _ if c.is_digit(10) => (
                        acc.0,
                        Some(acc.1.unwrap_or(0) * 10 + c.to_digit(10).unwrap()),
                    ),
                    _ => match acc {
                        (tokens, None) => (
                            [
                                &tokens[..],
                                &[Token::Symbol {
                                    coord: [x as i32, y as i32],
                                    value: c,
                                }],
                            ]
                            .concat(),
                            None,
                        ),
                        (tokens, Some(n)) => (
                            [
                                &tokens[..],
                                &[
                                    Token::Number {
                                        coord: [x as i32 - 1, y as i32],
                                        value: n,
                                    },
                                    Token::Symbol {
                                        coord: [x as i32, y as i32],
                                        value: c,
                                    },
                                ],
                            ]
                            .concat(),
                            None,
                        ),
                    },
                })
                .0
        })
        .collect();
    Map {
        numbers: tokens
            .iter()
            .filter_map(|token| match token {
                Token::Number { coord, value } => Some((*coord, *value)),
                _ => None,
            })
            .collect(),
        symbols: tokens
            .iter()
            .filter_map(|token| match token {
                Token::Symbol { coord, value } => Some((*coord, *value)),
                _ => None,
            })
            .collect(),
    }
}

fn part1(input: &Input) -> u32 {
    input
        .numbers
        .iter()
        .filter(|(number_coord, n)| {
            let digit_count = n.ilog10() as i32 + 1;
            ((-digit_count)..=1)
                .flat_map(|x_offset| {
                    [
                        [number_coord[0] + x_offset, number_coord[1] - 1],
                        [number_coord[0] + x_offset, number_coord[1] + 1],
                    ]
                })
                .chain([
                    [number_coord[0] - digit_count, number_coord[1]],
                    [number_coord[0] + 1, number_coord[1]],
                ])
                .any(|coord| input.symbols.contains_key(&coord))
        })
        .map(|(_coord, n)| n)
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .symbols
        .iter()
        .filter_map(|(symbol_coord, &c)| {
            if c != '*' {
                return None;
            }
            let adjacent_numbers: Vec<_> = input
                .numbers
                .iter()
                .filter_map(|(number_coord, &n)| {
                    if !((symbol_coord[1] - 1) <= number_coord[1]
                        && number_coord[1] <= (symbol_coord[1] + 1))
                    {
                        return None;
                    }
                    let digit_span = n.ilog10() as i32;
                    if (symbol_coord[0] - 1) <= number_coord[0]
                        && (number_coord[0] - digit_span) <= (symbol_coord[0] + 1)
                    {
                        return Some(n);
                    }
                    return None;
                })
                .collect();
            if adjacent_numbers.len() != 2 {
                return None;
            }
            Some(adjacent_numbers.iter().product::<u32>())
        })
        .sum()
}
