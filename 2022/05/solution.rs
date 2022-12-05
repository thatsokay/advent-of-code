use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
    let input = fs::read_to_string("input.txt").unwrap();
    let (cargo, instructions) = input.split_once("\n\n").unwrap();

    let cargo_rows: Vec<_> = cargo.lines().collect();
    let stack_count = (cargo_rows[0].len() + 1) / 4;
    let mut stacks = vec![vec![]; stack_count];
    cargo_rows
        .iter()
        .take(cargo_rows.len() - 1)
        .for_each(|row| {
            row.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(i, cargo_crate)| {
                    if cargo_crate != ' ' {
                        stacks[i].push(cargo_crate);
                    }
                });
        });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    let moves: Vec<_> = instructions
        .lines()
        .map(|line| {
            let move_values: Vec<_> = line
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            [move_values[0], move_values[1] - 1, move_values[2] - 1]
        })
        .collect();

    (stacks, moves)
}

fn part1(input: &(Vec<Vec<char>>, Vec<[usize; 3]>)) -> String {
    let mut stacks: Vec<_> = input.0.to_vec();
    input.1.iter().for_each(|&[n, source, destination]| {
        (0..n).for_each(|_| {
            let cargo_crate = stacks[source].pop().unwrap();
            stacks[destination].push(cargo_crate);
        })
    });
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(input: &(Vec<Vec<char>>, Vec<[usize; 3]>)) -> String {
    let mut stacks: Vec<_> = input.0.to_vec();
    input.1.iter().for_each(|&[n, source, destination]| {
        let range_start = stacks[source].len() - n;
        let cargo_crates: Vec<_> = stacks[source].drain(range_start..).collect();
        stacks[destination].extend(cargo_crates.iter());
    });
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
