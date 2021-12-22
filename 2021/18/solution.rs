use std::fmt;
use std::fs;

#[derive(Clone, Debug)]
enum Node {
    Leaf(u32),
    Branch(Box<Node>, Box<Node>),
}

use Node::*;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leaf(value) => write!(f, "{}", value),
            Branch(left_node, right_node) => {
                write!(f, "[{},{}]", left_node, right_node)
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Explosion {
    None,
    Exploded,
    Explode(u32, u32),
    ExplodeLeft(u32),
    ExplodeRight(u32),
}

#[derive(Clone, Debug)]
enum Split {
    None,
    Split,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<Box<Node>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| parse_snailfish_number(line).0)
        .collect()
}

fn part1(input: &[Box<Node>]) -> u32 {
    let sum = input
        .iter()
        .map(|node| (*node).clone())
        .reduce(|acc, node| {
            let mut head: Box<Node> = Box::new(Branch(acc, node));
            reduce(&mut head);
            head
        })
        .unwrap();
    magnitude(&sum)
}

fn part2(input: &[Box<Node>]) -> u32 {
    (0..input.len())
        .flat_map(|i| (0..input.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            if i == j {
                return 0;
            }
            let mut head: Box<Node> = Box::new(Branch(input[i].clone(), input[j].clone()));
            reduce(&mut head);
            magnitude(&head)
        })
        .max()
        .unwrap()
}

fn parse_snailfish_number(line: &str) -> (Box<Node>, &str) {
    match line.chars().next().unwrap() {
        '[' => {
            let (left, right_line) = parse_snailfish_number(&line[1..]);
            let (right, remaining) = parse_snailfish_number(&right_line[1..]);
            (Box::new(Branch(left, right)), &remaining[1..])
        }
        c => (Box::new(Leaf(c.to_digit(10).unwrap())), &line[1..]),
    }
}

fn reduce(node: &mut Box<Node>) {
    loop {
        match explode(node, 0) {
            Explosion::None => {}
            _ => continue,
        }
        match split(node) {
            Split::None => {}
            _ => continue,
        }
        break;
    }
}

fn explode(node: &mut Box<Node>, depth: u8) -> Explosion {
    match &mut **node {
        Leaf(_) => return Explosion::None,
        Branch(left_node, right_node) => {
            if depth >= 4 {
                if let (Leaf(left), Leaf(right)) = (&**left_node, &**right_node) {
                    let action = Explosion::Explode(*left, *right);
                    **node = Leaf(0);
                    return action;
                }
            }
            let left_action = explode(left_node, depth + 1);
            match left_action {
                Explosion::Exploded => return left_action,
                Explosion::ExplodeLeft(_) => return left_action,
                Explosion::ExplodeRight(value) => {
                    explode_side(right_node, value, false);
                    return Explosion::Exploded;
                }
                Explosion::Explode(left, right) => {
                    explode_side(right_node, right, false);
                    return Explosion::ExplodeLeft(left);
                }
                Explosion::None => {}
            }
            let right_action = explode(right_node, depth + 1);
            match right_action {
                Explosion::Exploded => return right_action,
                Explosion::ExplodeRight(_) => return right_action,
                Explosion::ExplodeLeft(value) => {
                    explode_side(left_node, value, true);
                    return Explosion::Exploded;
                }
                Explosion::Explode(left, right) => {
                    explode_side(left_node, left, true);
                    return Explosion::ExplodeRight(right);
                }
                Explosion::None => {}
            }
            Explosion::None
        }
    }
}

fn explode_side(node: &mut Box<Node>, explode_value: u32, left: bool) {
    match &mut **node {
        Leaf(value) => **node = Leaf(*value + explode_value),
        Branch(left_node, right_node) => {
            let next_node = if left { right_node } else { left_node };
            explode_side(next_node, explode_value, left);
        }
    }
}

fn split(node: &mut Box<Node>) -> Split {
    match &mut **node {
        Leaf(x) if *x >= 10 => {
            **node = Branch(Box::new(Leaf(*x / 2)), Box::new(Leaf((*x + 1) / 2)));
            return Split::Split;
        }
        Leaf(_) => return Split::None,
        Branch(left_node, right_node) => {
            let left_action = split(left_node);
            match left_action {
                Split::Split => return left_action,
                Split::None => {}
            }
            let right_action = split(right_node);
            match right_action {
                Split::Split => return right_action,
                Split::None => {}
            }
            Split::None
        }
    }
}

fn magnitude(node: &Box<Node>) -> u32 {
    match &**node {
        Leaf(value) => *value,
        Branch(left_node, right_node) => magnitude(left_node) * 3 + magnitude(right_node) * 2,
    }
}
