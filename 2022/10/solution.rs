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

fn part1(input: &[String]) -> i32 {
    let mut program_counter = 0;
    let mut register = 1;
    let mut sum = 0;
    let mut pending_add: Option<i32> = None;
    for i in 1..=220 {
        if program_counter >= input.len() {
            break;
        }
        if [20, 60, 100, 140, 180, 220].contains(&i) {
            sum += register * i as i32;
        }
        match pending_add {
            Some(add) => {
                register += add;
                pending_add = None;
                program_counter += 1;
            }
            None => match &input[program_counter][..4] {
                "noop" => {
                    program_counter += 1;
                }
                "addx" => {
                    pending_add = input[program_counter][5..].parse().ok();
                }
                _ => unreachable!(),
            },
        }
    }
    sum
}

fn part2(input: &[String]) -> String {
    let mut program_counter: usize = 0;
    let mut register: i32 = 1;
    let mut pending_add: Option<i32> = None;
    let mut crt = vec![];
    for i in 1..=240 {
        if program_counter >= input.len() {
            break;
        }
        if (((i - 1) % 40) - register).abs() <= 1 {
            crt.push('#');
        } else {
            crt.push(' ')
        }
        match pending_add {
            Some(add) => {
                register += add;
                pending_add = None;
                program_counter += 1;
            }
            None => match &input[program_counter][..4] {
                "noop" => {
                    program_counter += 1;
                }
                "addx" => {
                    pending_add = input[program_counter][5..].parse().ok();
                }
                _ => unreachable!(),
            },
        }
        if i % 40 == 0 {
            crt.push('\n');
        }
    }
    crt.into_iter().collect()
}
