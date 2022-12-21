use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Monkey {
    Literal(i64),
    Operation([String; 2], Operator),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expr {
    Literal(i64),
    Operation(Box<[Expr; 2]>, Operator),
    Variable,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> HashMap<String, Monkey> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (name, expr_str) = line.split_once(": ").unwrap();
            let expr_parse = expr_str.parse::<i64>();
            let expr = match expr_parse {
                Ok(x) => Monkey::Literal(x),
                Err(_) => {
                    let operation_tokens: Vec<_> = expr_str.split(' ').collect();
                    let operator = match operation_tokens[1] {
                        "+" => Operator::Add,
                        "-" => Operator::Subtract,
                        "*" => Operator::Multiply,
                        "/" => Operator::Divide,
                        _ => unreachable!(),
                    };
                    Monkey::Operation(
                        [
                            operation_tokens[0].to_string(),
                            operation_tokens[2].to_string(),
                        ],
                        operator,
                    )
                }
            };
            (name.to_string(), expr)
        })
        .collect()
}

fn part1(input: &HashMap<String, Monkey>) -> i64 {
    evaluate_monkey(input, "root")
}

fn part2(input: &HashMap<String, Monkey>) -> i64 {
    let [left_expr, right_expr] =
        if let Monkey::Operation([left_name, right_name], _) = &input["root"] {
            [
                evaluate_monkey_as_expr(input, left_name),
                evaluate_monkey_as_expr(input, right_name),
            ]
        } else {
            panic!()
        };
    let (mut value, mut variable_expr) = match [left_expr, right_expr] {
        [Expr::Literal(x), expr] => (x, expr),
        [expr, Expr::Literal(x)] => (x, expr),
        _ => panic!(),
    };
    while let Expr::Operation(operands, operator) = variable_expr {
        match *operands {
            [expr, Expr::Literal(x)] => {
                match operator {
                    Operator::Add => {
                        value -= x;
                    }
                    Operator::Subtract => {
                        value += x;
                    }
                    Operator::Multiply => {
                        value /= x;
                    }
                    Operator::Divide => {
                        value *= x;
                    }
                }
                variable_expr = expr;
            }
            [Expr::Literal(x), expr] => {
                match operator {
                    Operator::Add => {
                        value -= x;
                    }
                    Operator::Subtract => {
                        value = x - value;
                    }
                    Operator::Multiply => {
                        value /= x;
                    }
                    Operator::Divide => {
                        value = x / value;
                    }
                }
                variable_expr = expr;
            }
            _ => panic!(),
        }
    }
    value
}

fn evaluate_monkey(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    match &monkeys[name] {
        Monkey::Literal(x) => *x,
        Monkey::Operation(operand_names, operator) => {
            let evaluated_operands: Vec<_> = operand_names
                .iter()
                .map(|name| evaluate_monkey(monkeys, name))
                .collect();
            match operator {
                Operator::Add => evaluated_operands[0] + evaluated_operands[1],
                Operator::Subtract => evaluated_operands[0] - evaluated_operands[1],
                Operator::Multiply => evaluated_operands[0] * evaluated_operands[1],
                Operator::Divide => evaluated_operands[0] / evaluated_operands[1],
            }
        }
    }
}

fn evaluate_monkey_as_expr(monkeys: &HashMap<String, Monkey>, name: &str) -> Expr {
    if name == "humn" {
        return Expr::Variable;
    }
    match &monkeys[name] {
        Monkey::Literal(x) => Expr::Literal(*x),
        Monkey::Operation(operand_names, operator) => {
            let evaluated_operands = [
                evaluate_monkey_as_expr(monkeys, &operand_names[0]),
                evaluate_monkey_as_expr(monkeys, &operand_names[1]),
            ];
            match evaluated_operands {
                [Expr::Literal(a), Expr::Literal(b)] => match operator {
                    Operator::Add => Expr::Literal(a + b),
                    Operator::Subtract => Expr::Literal(a - b),
                    Operator::Multiply => Expr::Literal(a * b),
                    Operator::Divide => Expr::Literal(a / b),
                },
                _ => Expr::Operation(Box::new(evaluated_operands), *operator),
            }
        }
    }
}
