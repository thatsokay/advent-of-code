use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<i32> {
    let input = fs::read_to_string("input.txt").unwrap();
    let groups: Vec<_> = input.split("\n\n").collect();
    let mut group_totals: Vec<_> = groups
        .into_iter()
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    group_totals.sort_by(|a, &b| b.cmp(a));
    group_totals
}

fn part1(input: &[i32]) -> i32 {
    input[0]
}

fn part2(input: &[i32]) -> i32 {
    input.iter().take(3).sum()
}
