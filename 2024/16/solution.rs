use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::process;
use std::rc::Rc;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vector(i32, i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    coord: Vector,
    direction: Vector,
    steps: u32,
    turns: u32,
    prev: Option<Rc<Node>>,
}

#[derive(Debug, Clone)]
struct NodeIterator {
    current: Option<Rc<Node>>,
}

type Input = HashMap<Vector, char>;

impl Vector {
    const RIGHT: Self = Self(1, 0);
}

impl Vector {
    fn rotate_clockwise(self) -> Self {
        Self(-self.1, self.0)
    }

    fn rotate_anticlockwise(self) -> Self {
        Self(self.1, -self.0)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Node {
    fn new(coord: Vector, direction: Vector) -> Self {
        Self {
            coord,
            direction,
            steps: 0,
            turns: 0,
            prev: None,
        }
    }

    fn score(&self) -> u32 {
        self.turns * 1000 + self.steps
    }
}

impl NodeIterator {
    fn new(start: Option<Rc<Node>>) -> Self {
        Self { current: start }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score()
            .cmp(&self.score())
            .then(self.steps.cmp(&other.steps))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IntoIterator for Node {
    type Item = Rc<Node>;
    type IntoIter = NodeIterator;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator::new(Some(Rc::new(self)))
    }
}

impl Iterator for NodeIterator {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.current {
            let prev = current.prev.clone();
            self.current = prev.clone();
            prev
        } else {
            None
        }
    }
}

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (Vector(x as i32, y as i32), c))
        })
        .collect()
}

fn part1(input: &Input) -> u32 {
    let &end_coord = input.iter().find(|(_, c)| **c == 'E').unwrap().0;
    let &start_coord = input.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut heap = BinaryHeap::from([Node::new(start_coord, Vector::RIGHT)]);
    let mut visited = HashSet::new();
    while let Some(node) = heap.pop() {
        if node.coord == end_coord {
            return node.score();
        }
        if visited.contains(&(node.coord, node.direction)) {
            continue;
        }
        if let Some('#') = input.get(&node.coord) {
            continue;
        }
        visited.insert((node.coord, node.direction));
        let prev = Rc::new(node.clone());
        heap.push(Node {
            coord: node.coord + node.direction,
            direction: node.direction,
            steps: node.steps + 1,
            turns: node.turns,
            prev: Some(Rc::clone(&prev)),
        });
        heap.push(Node {
            coord: node.coord,
            direction: node.direction.rotate_clockwise(),
            turns: node.turns + 1,
            steps: node.steps,
            prev: Some(Rc::clone(&prev)),
        });
        heap.push(Node {
            coord: node.coord,
            direction: node.direction.rotate_anticlockwise(),
            turns: node.turns + 1,
            steps: node.steps,
            prev: Some(Rc::clone(&prev)),
        });
    }
    panic!("Path not found");
}

fn part2(input: &Input) -> usize {
    let &end_coord = input.iter().find(|(_, c)| **c == 'E').unwrap().0;
    let &start_coord = input.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut heap = BinaryHeap::from([Node::new(start_coord, Vector::RIGHT)]);
    let mut visited = HashMap::<(Vector, Vector), u32>::new();
    let mut seats = HashSet::new();
    let mut min_score = u32::MAX;
    while let Some(node) = heap.pop() {
        let score = node.score();
        if score > min_score {
            continue;
        }
        if node.coord == end_coord {
            min_score = score;
            seats.insert(node.coord);
            for path_node in node {
                seats.insert(path_node.coord);
            }
            continue;
        }
        if let Some(&prev_score) = visited.get(&(node.coord, node.direction)) {
            if prev_score < score {
                continue;
            }
        }
        if let Some('#') = input.get(&node.coord) {
            continue;
        }
        visited.insert((node.coord, node.direction), score);
        let prev = Rc::new(node.clone());
        heap.push(Node {
            coord: node.coord + node.direction,
            direction: node.direction,
            steps: node.steps + 1,
            turns: node.turns,
            prev: Some(Rc::clone(&prev)),
        });
        heap.push(Node {
            coord: node.coord,
            direction: node.direction.rotate_clockwise(),
            steps: node.steps,
            turns: node.turns + 1,
            prev: Some(Rc::clone(&prev)),
        });
        heap.push(Node {
            coord: node.coord,
            direction: node.direction.rotate_anticlockwise(),
            steps: node.steps,
            turns: node.turns + 1,
            prev: Some(Rc::clone(&prev)),
        });
    }
    seats.len()
}
