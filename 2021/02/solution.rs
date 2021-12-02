use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<(i32, i32)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("forward", magnitude) => (magnitude.parse::<i32>().unwrap(), 0),
            ("up", magnitude) => (0, magnitude.parse::<i32>().unwrap() * -1),
            ("down", magnitude) => (0, magnitude.parse::<i32>().unwrap()),
            _ => panic!("Invalid command"),
        })
        .collect()
}

fn part1(input: &[(i32, i32)]) -> i32 {
    let (x, y) = input
        .iter()
        .fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));
    x * y
}

fn part2(input: &[(i32, i32)]) -> i32 {
    let (_aim, x, y) = input.iter().fold((0, 0, 0), |(aim, acc_x, acc_y), (x, y)| {
        (aim + y, acc_x + x, acc_y + aim * x)
    });
    x * y
}
