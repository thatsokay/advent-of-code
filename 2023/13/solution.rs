use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Coord = [i32; 2];

type Map = HashSet<Coord>;

enum Mirror {
    Horizontal(i32),
    Vertical(i32),
}

type Input = Vec<Map>;

fn parse_input() -> Input {
    fs::read_to_string("test.txt")
        .unwrap()
        .split("\n\n")
        .map(|map| {
            map.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| match c {
                        '#' => Some([x as i32, y as i32]),
                        _ => None,
                    })
                })
                .collect()
        })
        .collect()
}

fn part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|map| match find_mirror(map) {
            Mirror::Horizontal(x) => x,
            Mirror::Vertical(y) => y * 100,
        })
        .sum()
}

fn part2(input: &Input) -> i32 {
    0
}

fn find_mirror(input: &Map) -> Mirror {
    let x_min = input.iter().min_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let x_max = input.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];

    let y_min = input.iter().min_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let y_max = input.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];

    ((x_min + 1)..x_max)
        .find(|mirror_x| {
            println!("Mirror at x = {}", mirror_x);
            let (left, right): (HashSet<&Coord>, HashSet<&Coord>) =
                input.iter().partition(|[x, _y]| x < mirror_x);
            left.iter().all(|[x, y]| {
                let mirrored_x = *mirror_x * 2 - x - 1;
                println!("{},{} -> {},{}", x, y, mirrored_x, y);
                *mirror_x <= x_max && right.contains(&[mirrored_x, *y])
            }) && right.iter().all(|[x, y]| {
                let mirrored_x = *mirror_x * 2 - x;
                *mirror_x >= x_min && left.contains(&[mirrored_x, *y])
            })
        })
        .and_then(|mirror_x| Some(Mirror::Horizontal(mirror_x)))
        .or_else(|| {
            ((y_min + 1)..y_max)
                .find(|mirror_y| {
                    let (up, down): (HashSet<&Coord>, HashSet<&Coord>) =
                        input.iter().partition(|[_x, y]| y < mirror_y);
                    up.iter().all(|[x, y]| {
                        let mirrored_y = *mirror_y * 2 - y;
                        *mirror_y <= y_max && down.contains(&[*x, mirrored_y])
                    }) && down.iter().all(|[x, y]| {
                        let mirrored_y = *mirror_y * 2 - y;
                        *mirror_y >= y_min && up.contains(&[*x, mirrored_y])
                    })
                })
                .and_then(|mirror_y| Some(Mirror::Vertical(mirror_y)))
        })
        .unwrap()
}
