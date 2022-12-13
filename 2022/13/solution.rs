use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

use Packet::*;

impl Packet {
    fn from(chars: &mut Peekable<Chars>) -> Packet {
        match chars.peek() {
            Some('[') => {
                chars.next();
                let mut list = vec![];
                loop {
                    match chars.peek() {
                        Some(']') => {
                            chars.next();
                            return List(list);
                        }
                        Some('[') => list.push(Self::from(chars)),
                        Some(',') => {
                            chars.next();
                        }
                        Some(c) if c.is_numeric() => list.push(Self::from(chars)),
                        _ => panic!(),
                    }
                }
            }
            Some(_) => {
                let mut value = 0;
                loop {
                    match chars.peek() {
                        Some(c) if c.is_numeric() => {
                            value = value * 10 + c.to_digit(10).unwrap();
                            chars.next();
                        }
                        _ => return Int(value),
                    }
                }
            }
            _ => panic!(),
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Int(x) => write!(f, "{}", x),
            List(xs) => {
                write!(
                    f,
                    "[{}]",
                    xs.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match [self, other] {
            [Int(x), Int(y)] => x.cmp(y),
            [List(xs), List(ys)] => xs
                .iter()
                .zip(ys.iter())
                .find_map(|(x, y)| match x.cmp(y) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                })
                .unwrap_or_else(|| xs.len().cmp(&ys.len())),
            [Int(x), ys @ List(_)] => List(vec![Int(*x)]).cmp(ys),
            [xs @ List(_), Int(y)] => xs.cmp(&List(vec![Int(*y)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match [self, other] {
            [Int(x), Int(y)] => x == y,
            [List(xs), List(ys)] => {
                xs.len() == ys.len() && xs.iter().zip(ys.iter()).all(|(x, y)| x == y)
            }
            _ => false,
        }
    }
}

impl Eq for Packet {}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<[Packet; 2]> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            let mut packets = group
                .lines()
                .map(|line| Packet::from(&mut line.chars().peekable()));
            [packets.next().unwrap(), packets.next().unwrap()]
        })
        .collect()
}

fn part1(input: &[[Packet; 2]]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, [a, b])| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &[[Packet; 2]]) -> usize {
    let mut packets: Vec<_> = input.iter().flat_map(|pair| pair.iter()).collect();
    packets.sort();
    let mut iter = packets.into_iter().enumerate();
    let i_1 = iter
        .find(|(_, packet)| packet > &&List(vec![List(vec![Int(2)])]))
        .unwrap()
        .0
        + 1;
    let i_2 = iter
        .find(|(_, packet)| packet > &&List(vec![List(vec![Int(6)])]))
        .unwrap()
        .0
        + 2;
    i_1 * i_2
}
