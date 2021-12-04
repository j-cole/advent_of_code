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

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let file = File::open("input/year_2021/day_03_1.txt").expect("Could not open input file.");
    let mut reader = BufReader::new(file);
    let binary_numbers = parse_input(&mut reader);
    get_life_support_rating(&binary_numbers)
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

fn get_life_support_rating(binary_numbers: &[String]) -> i64 {
    let oxygen_generator_rating = get_oxygen_generator_rating(binary_numbers);
    let co2_scrubber_rating = get_co2_scrubber_rating(binary_numbers);
    oxygen_generator_rating * co2_scrubber_rating
}

fn get_oxygen_generator_rating(binary_numbers: &[String]) -> i64 {
    let mut filter_numbers: Vec<String> = binary_numbers.to_vec();
    let mut pos = 0;
    while 1 < filter_numbers.len() {
        let most_common_bit = get_most_common_bit_at_pos(&filter_numbers, pos);
        filter_numbers = filter_at_pos(&filter_numbers, most_common_bit, pos);
        pos += 1;
    }
    i64::from_str_radix(&filter_numbers[0], 2).expect("Cannot parse filter number")
}

fn count_chars_at_pos(strings: &[String], pos: usize) -> HashMap<char, i64> {
    strings.iter().fold(HashMap::new(), |mut acc, elem| {
        if let Some(c) = elem.chars().nth(pos) {
            *acc.entry(c).or_insert(0) += 1;
        }
        acc
    })
}

fn get_most_common_bit_at_pos(strings: &[String], pos: usize) -> char {
    let count_chars = count_chars_at_pos(strings, pos);
    let count_zeros = count_chars.get(&'0').unwrap_or(&0);
    let count_ones = count_chars.get(&'1').unwrap_or(&0);
    if count_zeros <= count_ones {
        '1'
    } else {
        '0'
    }
}

fn filter_at_pos(strings: &[String], filter_value: char, pos: usize) -> Vec<String> {
    strings
        .iter()
        .filter(|bn| {
            if let Some(c) = bn.chars().nth(pos) {
                c == filter_value
            } else {
                false
            }
        })
        .map(String::from)
        .collect::<Vec<String>>()
}

fn get_least_common_bit_at_pos(strings: &[String], pos: usize) -> char {
    let count_chars = count_chars_at_pos(strings, pos);
    let count_zeros = count_chars.get(&'0').unwrap_or(&0);
    let count_ones = count_chars.get(&'1').unwrap_or(&0);
    if count_zeros <= count_ones {
        '0'
    } else {
        '1'
    }
}

fn get_co2_scrubber_rating(binary_numbers: &[String]) -> i64 {
    let mut filter_numbers: Vec<String> = binary_numbers.to_vec();
    let mut pos = 0;
    while 1 < filter_numbers.len() {
        let least_common_bit = get_least_common_bit_at_pos(&filter_numbers, pos);
        filter_numbers = filter_at_pos(&filter_numbers, least_common_bit, pos);
        pos += 1;
    }
    i64::from_str_radix(&filter_numbers[0], 2).expect("Cannot parse filter number")
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

    #[test]
    fn check_example_input_part_2_count_chars_at_pos() {
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
        let counts = count_chars_at_pos(&input, 0);
        assert_eq!(counts.get(&'1').unwrap(), &7);
        assert_eq!(counts.get(&'0').unwrap(), &5);
    }

    #[test]
    fn check_example_input_part_2_filter_most_common_bit() {
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
        let expected_output = vec![
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
        ];
        assert_eq!(filter_at_pos(&input, '1', 0), expected_output);
    }

    #[test]
    fn check_example_input_part_2_oxygen_generator_rating() {
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
        assert_eq!(get_oxygen_generator_rating(&input), 23);
    }

    #[test]
    fn check_example_input_part_2_co2_scrubber_rating() {
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
        assert_eq!(get_co2_scrubber_rating(&input), 10);
    }

    #[test]
    fn check_example_input_part_2() {
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
        assert_eq!(get_life_support_rating(&input), 230);
    }

    #[test]
    fn check_answer_part_2() {
        assert_eq!(part_2(), 6124992);
    }
}
