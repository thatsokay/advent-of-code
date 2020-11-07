use std::collections::HashMap;
use std::fs;

type Rectangle = (i32, i32, i32, i32, i32);
type LayerMap = HashMap<(i32, i32), i32>;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (Vec<Rectangle>, LayerMap) {
    let rectangles = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| -> Rectangle {
            let values: Vec<&str> = line.split(&[' ', '#', ',', ':', 'x'][..]).collect();
            (
                values[1].parse().unwrap(),
                values[3].parse().unwrap(),
                values[4].parse().unwrap(),
                values[6].parse().unwrap(),
                values[7].parse().unwrap(),
            )
        })
        .collect();
    let layer_map = count_layers(&rectangles);
    (rectangles, layer_map)
}

fn part1((_rectangles, layer_map): &(Vec<Rectangle>, LayerMap)) -> i32 {
    layer_map.values().filter(|layers| **layers > 1).count() as i32
}

fn part2((rectangles, layer_map): &(Vec<Rectangle>, LayerMap)) -> i32 {
    for rectangle in rectangles {
        if is_unique(rectangle, layer_map) {
            return rectangle.0;
        }
    }
    -1
}

fn count_layers(rectangles: &Vec<Rectangle>) -> LayerMap {
    let mut layers = HashMap::new();
    for (_id, x, y, w, h) in rectangles {
        for i in *x..(*x + *w) {
            for j in *y..(*y + *h) {
                layers
                    .entry((i, j))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    layers
}

fn is_unique(rectangle: &Rectangle, layer_map: &LayerMap) -> bool {
    let (_id, x, y, w, h) = rectangle;
    for i in *x..(*x + *w) {
        for j in *y..(*y + *h) {
            if layer_map.get(&(i, j)).unwrap() > &1 {
                return false;
            }
        }
    }
    true
}
