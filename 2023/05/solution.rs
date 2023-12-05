use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[derive(Debug, Clone)]
struct RangeMap {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Vec<RangeMap>>,
}

type Input = Almanac;

fn parse_input() -> Input {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let (seeds_description, mappings_description) = file_content.split_once("\n\n").unwrap();
    let seeds = seeds_description
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let mappings = mappings_description
        .split("\n\n")
        .map(|mapping_description| {
            mapping_description
                .lines()
                .skip(1)
                .map(|line| {
                    let mapping_parameters: Vec<i64> = line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
                    RangeMap {
                        source_start: mapping_parameters[1],
                        source_end: mapping_parameters[1] + mapping_parameters[2],
                        offset: mapping_parameters[0] - mapping_parameters[1],
                    }
                })
                .collect()
        })
        .collect();
    Almanac { seeds, mappings }
}

fn part1(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .map(|&seed| {
            input.mappings.iter().fold(seed, |source, mapping| {
                mapping
                    .iter()
                    .find(|range_map| {
                        range_map.source_start <= source && source < range_map.source_end
                    })
                    .and_then(|range_map| Some(range_map.offset))
                    .unwrap_or(0)
                    + source
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &Input) -> i64 {
    let seed_ranges: Vec<_> = input.seeds.chunks(2).collect();
    (0..)
        .find(|&location| {
            input
                .mappings
                .iter()
                .rev()
                .fold(HashSet::from([location]), |destinations, mapping| {
                    destinations
                        .into_iter()
                        .flat_map(|destination| {
                            let mut sources: HashSet<_> = mapping
                                .iter()
                                .filter_map(|range_map| {
                                    if range_map.source_start + range_map.offset <= destination
                                        && destination < range_map.source_end + range_map.offset
                                    {
                                        Some(destination - range_map.offset)
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if mapping.iter().all(|range_map| {
                                !(range_map.source_start <= destination
                                    && destination < range_map.source_end)
                            }) {
                                sources.insert(destination);
                            }
                            sources
                        })
                        .collect()
                })
                .iter()
                .any(|&seed| {
                    seed_ranges
                        .iter()
                        .any(|range| range[0] <= seed && seed < range[0] + range[1])
                })
        })
        .unwrap()
}
