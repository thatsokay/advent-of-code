use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Input = Vec<(u64, u64)>;

fn parse_input() -> Input {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let mut rows = file_content
        .lines()
        .map(|line| line.split_whitespace().skip(1).map(|x| x.parse().unwrap()));
    let time_numbers = rows.next().unwrap();
    let distance_numbers = rows.next().unwrap();
    time_numbers.zip(distance_numbers).collect()
}

fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|&(time, distance)| count_winning_ways(time, distance))
        .product()
}

fn part2(input: &Input) -> u64 {
    let (time, distance) =
        input
            .iter()
            .fold((0, 0), |(acc_time, acc_distance), &(time, distance)| {
                let time_digits = time.ilog10() + 1;
                let distance_digits = distance.ilog10() + 1;
                (
                    acc_time * 10_u64.pow(time_digits) + time,
                    acc_distance * 10_u64.pow(distance_digits) + distance,
                )
            });
    count_winning_ways(time, distance)
}

fn count_winning_ways(time: u64, distance: u64) -> u64 {
    // charge^2 - time * charge + distance = 0
    let lower_root = (time as f64 - ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.0;
    let upper_root = (time as f64 + ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.0;
    let min_charge_time = lower_root.floor() as u64;
    let max_charge_time = upper_root.ceil() as u64;
    max_charge_time - min_charge_time - 1
}
