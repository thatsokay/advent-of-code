use std::collections::{BTreeSet, HashMap};
use std::fs;

type Entry = (Vec<BTreeSet<char>>, Vec<BTreeSet<char>>);

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Entry> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (signals, output) = line.split_once(" | ").unwrap();
            (
                signals.split(' ').map(|s| s.chars().collect()).collect(),
                output.split(' ').map(|s| s.chars().collect()).collect(),
            )
        })
        .collect()
}

fn part1(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|entry| {
            entry
                .1
                .iter()
                .filter(|output| {
                    let segment_length = output.len();
                    segment_length == 2
                        || segment_length == 3
                        || segment_length == 4
                        || segment_length == 7
                })
                .count() as u32
        })
        .sum()
}

fn part2(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|(signals, output)| {
            let mut signal_map: HashMap<BTreeSet<char>, char> = HashMap::new();

            let one = signals
                .iter()
                .find(|signal| signal.len() == 2)
                .unwrap()
                .clone();
            let seven = signals
                .iter()
                .find(|signal| signal.len() == 3)
                .unwrap()
                .clone();
            let four = signals
                .iter()
                .find(|signal| signal.len() == 4)
                .unwrap()
                .clone();
            let eight = signals
                .iter()
                .find(|signal| signal.len() == 7)
                .unwrap()
                .clone();
            let two = signals
                .iter()
                .find(|signal| signal.len() == 5 && signal.difference(&four).count() == 3)
                .unwrap()
                .clone();
            let three = signals
                .iter()
                .find(|signal| signal.len() == 5 && signal.difference(&one).count() == 3)
                .unwrap()
                .clone();
            let five = signals
                .iter()
                .find(|signal| signal.len() == 5 && signal.difference(&two).count() == 2)
                .unwrap()
                .clone();
            let zero = signals
                .iter()
                .find(|signal| signal.len() == 6 && signal.difference(&five).count() == 2)
                .unwrap()
                .clone();
            let six = signals
                .iter()
                .find(|signal| signal.len() == 6 && signal.difference(&seven).count() == 4)
                .unwrap()
                .clone();
            let nine = signals
                .iter()
                .find(|signal| signal.len() == 6 && signal.difference(&four).count() == 2)
                .unwrap()
                .clone();

            signal_map.insert(zero, '0');
            signal_map.insert(one, '1');
            signal_map.insert(two, '2');
            signal_map.insert(three, '3');
            signal_map.insert(four, '4');
            signal_map.insert(five, '5');
            signal_map.insert(six, '6');
            signal_map.insert(seven, '7');
            signal_map.insert(eight, '8');
            signal_map.insert(nine, '9');

            output
                .iter()
                .map(|signal| signal_map.get(signal).unwrap())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}
