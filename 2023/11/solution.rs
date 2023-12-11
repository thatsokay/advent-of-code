use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Coord = [i64; 2];

type Input = HashSet<Coord>;

fn parse_input() -> Input {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some([x as i64, y as i64]),
                _ => None,
            })
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    let expanded: Vec<_> = expand_universe(input, 2).into_iter().collect();
    (0..(expanded.len()))
        .flat_map(|i| (i..(expanded.len())).map(move |j| [i, j]))
        .map(|[i, j]| {
            let galaxies = [expanded[i], expanded[j]];
            (galaxies[0][0] - galaxies[1][0]).abs() + (galaxies[0][1] - galaxies[1][1]).abs()
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    let expanded: Vec<_> = expand_universe(input, 1_000_000).into_iter().collect();
    (0..(expanded.len()))
        .flat_map(|i| (i..(expanded.len())).map(move |j| [i, j]))
        .map(|[i, j]| {
            let galaxies = [expanded[i], expanded[j]];
            (galaxies[0][0] - galaxies[1][0]).abs() + (galaxies[0][1] - galaxies[1][1]).abs()
        })
        .sum()
}

fn expand_universe(input: &Input, expanded_size: i64) -> Input {
    let x_min = input.iter().min_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let x_max = input.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];

    let y_min = input.iter().min_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let y_max = input.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];

    let empty_rows: Vec<_> = (x_min..x_max)
        .filter(|x| input.iter().all(|[galaxy_x, _]| galaxy_x != x))
        .collect();
    let empty_cols: Vec<_> = (y_min..y_max)
        .filter(|y| input.iter().all(|[_, galaxy_y]| galaxy_y != y))
        .collect();

    input
        .iter()
        .map(|[x, y]| {
            let row_expansion = match empty_rows.binary_search(x) {
                Ok(x) => x,
                Err(x) => x,
            } as i64
                * (expanded_size - 1);
            let col_expansion = match empty_cols.binary_search(y) {
                Ok(y) => y,
                Err(y) => y,
            } as i64
                * (expanded_size - 1);
            [x + row_expansion, y + col_expansion]
        })
        .collect()
}
