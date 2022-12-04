use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> Vec<[u32; 4]> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let section_ids: Vec<_> = line
                .split(',')
                .flat_map(|range| {
                    range
                        .split('-')
                        .map(|section| section.parse::<u32>().unwrap())
                })
                .collect();
            [
                section_ids[0],
                section_ids[1],
                section_ids[2],
                section_ids[3],
            ]
        })
        .collect()
}

fn part1(input: &[[u32; 4]]) -> u32 {
    input
        .iter()
        .filter(|section_ids| {
            (section_ids[0] <= section_ids[2] && section_ids[1] >= section_ids[3])
                || (section_ids[0] >= section_ids[2] && section_ids[1] <= section_ids[3])
        })
        .count() as u32
}

fn part2(input: &[[u32; 4]]) -> u32 {
    input
        .iter()
        .filter(|section_ids| {
            (section_ids[0] >= section_ids[2] || section_ids[1] >= section_ids[2])
                && (section_ids[0] <= section_ids[3] || section_ids[1] <= section_ids[3])
        })
        .count() as u32
}
