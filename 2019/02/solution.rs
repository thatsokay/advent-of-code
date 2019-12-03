use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input).unwrap());
}

fn parse_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    run_intcode(input, 12, 2)
}

fn part2(input: &Vec<i32>) -> Option<i32> {
    for noun in 1..100 {
        for verb in 1..100 {
            if run_intcode(input, noun, verb) == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }
    return None;
}

fn run_intcode(intcode: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut memory = intcode.clone();
    memory[1] = noun;
    memory[2] = verb;
    let mut ip: usize = 0;

    loop {
        match memory[ip] {
            1 => operate(&mut memory, &ip, |x, y| x + y),
            2 => operate(&mut memory, &ip, |x, y| x * y),
            99 => break,
            _ => panic!("Unknown operation"),
        }
        ip += 4;
    }
    memory[0]
}

fn operate(memory: &mut Vec<i32>, ip: &usize, op: fn(i32, i32) -> i32) {
    let params: Vec<usize> = (1..4_usize).map(|i| memory[ip + i] as usize).collect();
    memory[params[2]] = op(memory[params[0]], memory[params[1]]);
}
