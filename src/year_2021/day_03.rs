use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let file = File::open("input/year_2021/day_03_1.txt").expect("Could not open input file.");
    let mut reader = BufReader::new(file);
    let binary_numbers = parse_input(&mut reader);
    find_power_consumption(&binary_numbers)
}

fn parse_input<R: BufRead>(reader: &mut R) -> Vec<String> {
    reader
        .lines()
        .map(|line| line.expect("Could not read line.").trim().to_string())
        .collect::<Vec<String>>()
}

fn find_power_consumption(binary_numbers: &[String]) -> i64 {
    let ratio: Vec<i64> =
        binary_numbers
            .iter()
            .fold(vec![0; binary_numbers[0].len()], |acc, elem| {
                acc.iter()
                    .zip(elem.chars())
                    .map(|(a, c)| if c == '1' { a + 1 } else { a - 1 })
                    .collect()
            });

    let gamma = get_gamma(&ratio);
    let epsilon = get_epsilon(&ratio);
    gamma * epsilon
}

fn get_gamma(ratio: &[i64]) -> i64 {
    let most_common_bits: String = ratio
        .iter()
        .map(|&r| if 0 < r { '1' } else { '0' })
        .collect();
    i64::from_str_radix(&most_common_bits, 2).expect("Could not parse gamma")
}
fn get_epsilon(ratio: &[i64]) -> i64 {
    let least_common_bits: String = ratio
        .iter()
        .map(|&r| if r < 0 { '1' } else { '0' })
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
        assert_eq!(find_power_consumption(&input), 198);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 1071734);
    }
}
