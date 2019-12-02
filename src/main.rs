use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day_1_1();
}

fn day_1_1() {
    let file = File::open("input/2019/day_1_1.txt")
        .expect("Could not open input file.");
    let reader = BufReader::new(file);
    let result: u64 = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|u| fuel_for_module(u))
        .sum();
    println!("Sum of fuel requirements: {}", result);
}

fn fuel_for_module(mass: u64) -> u64 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(fuel_for_module(12), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(fuel_for_module(14), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(fuel_for_module(1969), 654);
    }

    #[test]
    fn example_4() {
        assert_eq!(fuel_for_module(100756), 33583);
    }
}
