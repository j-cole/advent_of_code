use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let file = File::open("input/year_2021/day_03_1.txt").expect("Could not open input file.");
    let mut reader = BufReader::new(file);
    let binary_numbers = parse_input(&mut reader);
    get_power_consumption(&binary_numbers)
}

fn parse_input<R: BufRead>(reader: &mut R) -> Vec<String> {
    reader
        .lines()
        .map(|line| line.expect("Could not read line.").trim().to_string())
        .collect::<Vec<String>>()
}

fn get_power_consumption(binary_numbers: &[String]) -> i64 {
    let counts = count_chars(binary_numbers);
    let gamma = get_gamma(&counts);
    let epsilon = get_epsilon(&counts);
    gamma * epsilon
}

fn count_chars(strings: &[String]) -> Vec<HashMap<char, i64>> {
    strings
        .iter()
        .fold(vec![HashMap::new(); strings[0].len()], |mut acc, elem| {
            acc.iter_mut()
                .zip(elem.chars())
                .for_each(|(a, c)| *a.entry(c).or_insert(0) += 1);
            acc
        })
}

fn get_gamma(counts: &[HashMap<char, i64>]) -> i64 {
    let most_common_bits: String = counts
        .iter()
        .map(|c| {
            let count_zeros = c.get(&'0').unwrap_or(&0);
            let count_ones = c.get(&'1').unwrap_or(&0);
            if count_zeros < count_ones {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    i64::from_str_radix(&most_common_bits, 2).expect("Could not parse gamma")
}

fn get_epsilon(counts: &[HashMap<char, i64>]) -> i64 {
    let least_common_bits: String = counts
        .iter()
        .map(|c| {
            let count_zeros = c.get(&'0').unwrap_or(&0);
            let count_ones = c.get(&'1').unwrap_or(&0);
            if count_ones < count_zeros {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    i64::from_str_radix(&least_common_bits, 2).expect("Could not parse epsilon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_input_part_1() {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(get_power_consumption(&input), 198);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 1071734);
    }
}
