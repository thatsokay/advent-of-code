use std::collections::HashSet;
use std::fs;

type Dots = HashSet<[i32; 2]>;
enum Fold {
    X(i32),
    Y(i32),
}
struct Paper {
    dots: Dots,
    folds: Vec<Fold>,
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Paper {
    let input = fs::read_to_string("input.txt").unwrap();
    let (dots_raw, folds_raw) = input.split_once("\n\n").unwrap();
    let dots: Dots = dots_raw
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();
    let folds: Vec<Fold> = folds_raw
        .lines()
        .map(|line| {
            let (axis, midpoint) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            match axis {
                "x" => Fold::X(midpoint.parse().unwrap()),
                "y" => Fold::Y(midpoint.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();
    Paper { dots, folds }
}

fn part1(input: &Paper) -> u32 {
    fold_paper(&input.dots, &input.folds[..1]).len() as u32
}

fn part2(input: &Paper) -> String {
    display_dots(&fold_paper(&input.dots, &input.folds))
}

fn fold_paper(dots: &Dots, folds: &[Fold]) -> Dots {
    let mut result = dots.clone();

    let mut flip_dots = |axis: usize, midpoint: i32| {
        let to_flip: Vec<_> = result
            .iter()
            .filter(|&coord| coord[axis] > midpoint)
            .map(|dot| dot.clone())
            .collect();
        for mut coord in to_flip {
            result.remove(&coord);
            coord[axis] = ((coord[axis] - midpoint) * -1) + midpoint;
            result.insert(coord);
        }
    };

    for fold in folds {
        match fold {
            Fold::X(midpoint) => {
                flip_dots(0, *midpoint);
            }
            Fold::Y(midpoint) => {
                flip_dots(1, *midpoint);
            }
        }
    }
    result
}

fn display_dots(dots: &Dots) -> String {
    let max_x = dots.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let max_y = dots.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| if dots.contains(&[x, y]) { '#' } else { ' ' })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
