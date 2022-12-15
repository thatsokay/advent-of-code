use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> HashMap<[i32; 2], [i32; 2]> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let values: Vec<_> = line
                .replace(['=', ',', ':'], " ")
                .split(' ')
                .filter_map(|word| word.parse::<i32>().ok())
                .collect();
            ([values[0], values[1]], [values[2], values[3]])
        })
        .collect()
}

fn part1(input: &HashMap<[i32; 2], [i32; 2]>) -> usize {
    input
        .iter()
        .flat_map(|(sensor, beacon)| {
            let distance = (sensor[0] - beacon[0]).abs() + (sensor[1] - beacon[1]).abs();
            let distance_at_row = distance - (sensor[1] - 2000000).abs();
            (sensor[0] - distance_at_row)..=(sensor[0] + distance_at_row)
        })
        .collect::<HashSet<_>>()
        .len()
        - input
            .values()
            .filter(|beacon| beacon[1] == 2000000)
            .map(|beacon| beacon[0])
            .collect::<HashSet<_>>()
            .len()
}

fn part2(input: &HashMap<[i32; 2], [i32; 2]>) -> u64 {
    let [x, y] = (0..=4000000)
        .find_map(|y| {
            let mut sensor_ranges_at_row: Vec<_> = input
                .iter()
                .flat_map(|(sensor, beacon)| {
                    let distance = (sensor[0] - beacon[0]).abs() + (sensor[1] - beacon[1]).abs();
                    let distance_at_row = distance - (sensor[1] - y).abs();
                    if distance_at_row < 0 {
                        None
                    } else {
                        Some([
                            sensor[0] - distance_at_row,
                            sensor[0] + distance_at_row + 1, // Exclusive end bound
                        ])
                    }
                })
                .collect();
            sensor_ranges_at_row.sort();
            sensor_ranges_at_row
                .iter()
                .enumerate()
                .skip(1) // Must have at least one range before it to compare with
                .find(|&(i, &[start, _])| {
                    if (0..=4000001).contains(&start) {
                        sensor_ranges_at_row
                            .iter()
                            .take(i)
                            .all(|&[_, end]| end < start)
                    } else {
                        false
                    }
                })
                .map(|(_, &[start, _])| [start - 1, y])
        })
        .unwrap();
    x as u64 * 4000000 + y as u64
}
