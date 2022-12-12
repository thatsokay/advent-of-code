use std::collections::{HashMap, VecDeque};
use std::fs;

type Coord = [i32; 2];

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (HashMap<Coord, i8>, HashMap<Coord, u32>, Coord) {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut height_map = HashMap::new();
    let mut start_coord = [0; 2];
    let mut end_coord = [0; 2];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let coord = [x as i32, y as i32];
            match c {
                'S' => {
                    start_coord = coord;
                    height_map.insert(coord, 'a' as i8);
                }
                'E' => {
                    end_coord = coord;
                    height_map.insert(coord, 'z' as i8);
                }
                _ => {
                    height_map.insert(coord, c as i8);
                }
            }
        })
    });
    let steps_map = map_steps_to_end(&height_map, &end_coord);
    (height_map, steps_map, start_coord)
}

fn part1(input: &(HashMap<Coord, i8>, HashMap<Coord, u32>, Coord)) -> u32 {
    input.1[&input.2]
}

fn part2(input: &(HashMap<Coord, i8>, HashMap<Coord, u32>, Coord)) -> u32 {
    *input
        .0
        .iter()
        .filter(|(_, height)| **height == 'a' as i8)
        .flat_map(|(coord, _)| input.1.get(coord))
        .min()
        .unwrap()
}

fn map_steps_to_end(height_map: &HashMap<Coord, i8>, end: &Coord) -> HashMap<Coord, u32> {
    let mut steps_map = HashMap::from([(*end, 0)]);
    let mut queue = VecDeque::from([*end]);
    while let Some(current_coord @ [x, y]) = queue.pop_front() {
        let current_height = height_map[&[x, y]];
        for coord in [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]] {
            if let Some(height) = height_map.get(&coord) {
                if (current_height - height) <= 1 && !steps_map.contains_key(&coord) {
                    let current_steps = steps_map[&current_coord];
                    steps_map.insert(coord, current_steps + 1);
                    queue.push_back(coord);
                }
            }
        }
    }
    steps_map
}
