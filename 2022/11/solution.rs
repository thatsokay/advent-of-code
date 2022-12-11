use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    throw_true: usize,
    throw_false: usize,
}

#[derive(Debug)]
struct Operation {
    operands: [Operand; 2],
    operator: Operator,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Literal(u64),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Monkey> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            let lines: Vec<_> = group.lines().collect();
            let items: Vec<_> = lines[1]
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let operation_tokens: Vec<_> =
                lines[2].split(" = ").last().unwrap().split(' ').collect();
            let operands: Vec<_> = [operation_tokens[0], operation_tokens[2]]
                .iter()
                .map(|&operand| match operand {
                    "old" => Operand::Old,
                    _ => Operand::Literal(operand.parse().unwrap()),
                })
                .collect();
            let operator = match operation_tokens[1] {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => unreachable!(),
            };
            let operation = Operation {
                operands: [operands[0], operands[1]],
                operator,
            };
            let test_divisible_by = lines[3].split(' ').last().unwrap().parse().unwrap();
            let throw_true = lines[4].split(' ').last().unwrap().parse().unwrap();
            let throw_false = lines[5].split(' ').last().unwrap().parse().unwrap();
            Monkey {
                items,
                operation,
                test_divisible_by,
                throw_true,
                throw_false,
            }
        })
        .collect()
}

fn part1(input: &[Monkey]) -> usize {
    monkey_around(input, 20, None)
}

fn part2(input: &[Monkey]) -> usize {
    let modulo = input
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product();
    monkey_around(input, 10000, Some(modulo))
}

fn monkey_around(monkeys: &[Monkey], rounds: usize, modulo: Option<u64>) -> usize {
    let initial_state: Vec<_> = monkeys
        .iter()
        .map(|monkey| (0, monkey.items.clone()))
        .collect();
    let mut final_state = (0..rounds).fold(initial_state, |mut acc, _i| {
        (0..monkeys.len()).for_each(|monkey_index| {
            let monkey = &monkeys[monkey_index];
            let items_len = acc[monkey_index].1.len();
            (0..items_len).for_each(|item_index| {
                let item_worry = acc[monkey_index].1[item_index];
                let new_item = new_item_worry(item_worry, &monkey.operation, modulo);
                let test_result = new_item % monkey.test_divisible_by == 0;
                let throw_to = if test_result {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };
                acc[throw_to].1.push(new_item);
            });
            acc[monkey_index].0 += items_len;
            acc[monkey_index].1 = vec![];
        });
        acc
    });
    final_state.sort_by(|a, b| b.0.cmp(&a.0));
    final_state[0].0 * final_state[1].0
}

fn new_item_worry(item_worry: u64, operation: &Operation, modulo: Option<u64>) -> u64 {
    let operands: Vec<_> = operation
        .operands
        .iter()
        .map(|operand| match operand {
            Operand::Old => item_worry,
            Operand::Literal(value) => *value,
        })
        .collect();
    let result = match operation.operator {
        Operator::Add => operands[0] + operands[1],
        Operator::Multiply => operands[0] * operands[1],
    };
    match modulo {
        Some(value) => result % value,
        None => result / 3,
    }
}
