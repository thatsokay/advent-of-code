use std::fs;

fn run_intcode(intcode: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
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
    memory
}

fn operate(memory: &mut Vec<i32>, ip: &usize, op: fn(i32, i32) -> i32) {
    let params: Vec<usize> = (1..4_usize).map(|i| memory[ip + i] as usize).collect();
    memory[params[2]] = op(memory[params[0]], memory[params[1]]);
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    let part1 = run_intcode(&input, 12, 2);
    println!("{}", part1[0]);

    for i in 1..100 {
        for j in 1..100 {
            if run_intcode(&input, i, j)[0] == 19690720 {
                println!("{}", i * 100 + j);
                return;
            }
        }
    }
}
