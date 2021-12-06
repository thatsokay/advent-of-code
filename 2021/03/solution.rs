use std::fs;

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (Vec<u32>, usize) {
    let input = fs::read_to_string("input.txt").unwrap();
    let bit_count = input.lines().next().unwrap().len();
    let numbers = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    (numbers, bit_count)
}

fn part1((numbers, bit_count): &(Vec<u32>, usize)) -> u32 {
    let majority_min_size = numbers.len() / 2;
    let mut counter = vec![0; *bit_count];
    for number in numbers {
        for i in 0..*bit_count {
            let bit_shift = bit_count - i - 1;
            if number & (1 << bit_shift) == 0 {
                counter[i] += 1;
            }
        }
    }

    let epsilon = counter.iter().fold(0, |acc, &count| {
        let majority_0 = count > majority_min_size;
        acc * 2 + (majority_0 as u32)
    });
    let gamma = epsilon ^ (2_u32.pow(*bit_count as u32) - 1);
    gamma * epsilon
}

fn part2((numbers, bit_count): &(Vec<u32>, usize)) -> u32 {
    let oxygen_rating_options: Vec<u32> = numbers.clone();
    let co2_rating_options: Vec<u32> = numbers.clone();
    let oxygen_rating = filter_numbers(oxygen_rating_options, bit_count - 1, true);
    let co2_rating = filter_numbers(co2_rating_options, bit_count - 1, false);
    oxygen_rating * co2_rating
}

fn filter_numbers(numbers: Vec<u32>, bit_shift: usize, majority_criteria: bool) -> u32 {
    let zero_count = numbers
        .iter()
        .filter(|&&number| number & (1 << bit_shift) == 0)
        .count();
    let majority_min_size = numbers.len() / 2;
    let majority_bit = (zero_count <= majority_min_size) as u32;
    let filter_bit = majority_bit ^ (!majority_criteria as u32);
    let filtered: Vec<_> = numbers
        .into_iter()
        .filter(|number| number & (1 << bit_shift) == filter_bit << bit_shift)
        .collect();
    if filtered.len() == 1 {
        return filtered[0];
    }
    filter_numbers(filtered, bit_shift - 1, majority_criteria)
}
