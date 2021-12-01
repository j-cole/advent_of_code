use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() {
    let file = File::open("input/year_2021/day_01_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let input: Vec<u64> = reader
        .lines()
        .map(|line| {
            line.expect("Could not read line.")
                .parse::<u64>()
                .expect("Could not parse line")
        })
        .collect();
    let result = count_depth_increases(&input);
    println!("Number of depth increases part 1: {}", result);
}

#[allow(dead_code)]
pub fn part_2() {
    let file = File::open("input/year_2021/day_01_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let input: Vec<u64> = reader
        .lines()
        .map(|line| {
            line.expect("Could not read line.")
                .parse::<u64>()
                .expect("Could not parse line")
        })
        .collect();
    let result = count_depth_increases_by_3(&input);
    println!("Number of depth increases part 2: {}", result);
}

fn count_depth_increases(depths: &[u64]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_depth_increases_by_3(depths: &[u64]) -> usize {
    depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_depths_returns_zero() {
        let depths: Vec<u64> = vec![];
        assert_eq!(count_depth_increases(&depths), 0);
    }

    #[test]
    fn single_depth_returns_zero() {
        let depths: Vec<u64> = vec![10];
        assert_eq!(count_depth_increases(&depths), 0);
    }

    #[test]
    fn increase_followed_by_decrease_depth_returns_one() {
        let depths: Vec<u64> = vec![10, 20, 15];
        assert_eq!(count_depth_increases(&depths), 1);
    }

    #[test]
    fn decrease_followed_by_increase_depth_returns_one() {
        let depths: Vec<u64> = vec![10, 5, 20];
        assert_eq!(count_depth_increases(&depths), 1);
    }

    #[test]
    fn test_example_input_part_1() {
        let depths: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&depths), 7);
    }

    #[test]
    fn test_example_input_part_2() {
        let depths: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases_by_3(&depths), 5);
    }
}
