use std::fs;

fn fuel(mass: &i32) -> i32 {
    let mut fuel_mass = *mass;
    let mut total_fuel = 0;
    while fuel_mass > 6 {
        fuel_mass = fuel_mass / 3 - 2;
        total_fuel += fuel_mass;
    }
    total_fuel
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Failed reading file")
        .lines()
        .map(|line| line.parse().expect("Failed parsing line into integer"))
        .collect();

    let part1 = input.iter().fold(0, |acc, mass| acc + (mass / 3 - 2));
    println!("{}", part1);

    let part2 = input.iter().fold(0, |acc, mass| acc + fuel(mass));
    println!("{}", part2);
}
