use std::collections::HashMap;
use std::fs;

struct Forest {
    heights: HashMap<[usize; 2], u8>,
    x_len: usize,
    y_len: usize,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Forest {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let y_len = lines.len();
    let x_len = lines[0].len();
    let heights = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ([x, y], c.to_digit(10).unwrap() as u8))
        })
        .collect();
    Forest {
        heights,
        x_len,
        y_len,
    }
}

fn part1(input: &Forest) -> usize {
    input
        .heights
        .keys()
        .filter(|coord| is_visible(coord, input))
        .count()
}

fn part2(input: &Forest) -> usize {
    input
        .heights
        .keys()
        .map(|coord| scenic_score(coord, input))
        .max()
        .unwrap()
}

fn is_visible(coord: &[usize; 2], forest: &Forest) -> bool {
    let height = forest.heights.get(coord).unwrap();
    (0..coord[0]).all(|x| forest.heights.get(&[x, coord[1]]).unwrap() < height)
        || ((coord[0] + 1)..forest.x_len)
            .all(|x| forest.heights.get(&[x, coord[1]]).unwrap() < height)
        || (0..coord[1]).all(|y| forest.heights.get(&[coord[0], y]).unwrap() < height)
        || ((coord[1] + 1)..forest.y_len)
            .all(|y| forest.heights.get(&[coord[0], y]).unwrap() < height)
}

fn scenic_score(coord: &[usize; 2], forest: &Forest) -> usize {
    let height = forest.heights.get(coord).unwrap();
    let score_left = coord[0]
        - (0..coord[0])
            .rev()
            .find(|&x| forest.heights.get(&[x, coord[1]]).unwrap() >= height)
            .unwrap_or(0);
    let score_right = ((coord[0] + 1)..forest.x_len)
        .find(|&x| forest.heights.get(&[x, coord[1]]).unwrap() >= height)
        .unwrap_or(forest.x_len - 1)
        - coord[0];
    let score_up = coord[1]
        - (0..coord[1])
            .rev()
            .find(|&y| forest.heights.get(&[coord[0], y]).unwrap() >= height)
            .unwrap_or(0);
    let score_down = ((coord[1] + 1)..forest.y_len)
        .find(|&y| forest.heights.get(&[coord[0], y]).unwrap() >= height)
        .unwrap_or(forest.y_len - 1)
        - coord[1];
    score_left * score_right * score_up * score_down
}
