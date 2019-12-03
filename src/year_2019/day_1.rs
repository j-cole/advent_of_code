use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() {
    let file = File::open("input/2019/day_1_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let result: u64 = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|u| fuel_for_mass(u))
        .sum();
    println!("Sum of fuel requirements part 1: {}", result);
}

#[allow(dead_code)]
pub fn part_2() {
    let file = File::open("input/2019/day_1_2.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let result: u64 = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|u| fuel_for_mass_with_extra_fuel(u))
        .sum();
    println!("Sum of fuel requirements part 2: {}", result);
}

fn fuel_for_mass(mass: u64) -> u64 {
    let fuel = (mass / 3) as i64 - 2;
    if 0 < fuel {
        fuel as u64
    } else {
        0
    }
}

fn fuel_for_mass_with_extra_fuel(mass: u64) -> u64 {
    let mut total_fuel = 0;
    let mut extra_fuel = fuel_for_mass(mass);
    while 0 < extra_fuel {
        total_fuel = total_fuel + extra_fuel;
        extra_fuel = fuel_for_mass(extra_fuel);
    }
    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        assert_eq!(fuel_for_mass(12), 2);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(fuel_for_mass(14), 2);
    }

    #[test]
    fn example_1_3() {
        assert_eq!(fuel_for_mass(1969), 654);
    }

    #[test]
    fn example_1_4() {
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn example_1_5() {
        assert_eq!(fuel_for_mass_with_extra_fuel(14), 2);
    }

    #[test]
    fn example_1_6() {
        assert_eq!(fuel_for_mass_with_extra_fuel(1969), 966);
    }

    #[test]
    fn example_1_7() {
        assert_eq!(fuel_for_mass_with_extra_fuel(100756), 50346);
    }
}
