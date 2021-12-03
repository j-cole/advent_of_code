use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let file = File::open("input/year_2021/day_03_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| {
            line.expect("Could not read line.").trim().into()
        })
        .collect::<Vec<String>>();
    find_gamma_multiplied_by_epsilon(&lines)
}

fn find_gamma_multiplied_by_epsilon(lines: &[String]) -> i64{
    let mut counts: Vec<i64> = vec![0; lines[0].len()];
    for l in lines.iter() {
        for (i, c) in l.chars().enumerate() {
            println!("char: {} , pos: {}", c, i);
            if c == '1' {
                counts[i] += 1;
            } else {
                counts[i] -= 1;
            }
        }

    }
    let mut gamma = 0;
    for &c in &counts {
        gamma *= 2;
        if 0 < c {
            gamma +=1;
        }
    }
    let mut epsilon = 0;
    for &c in &counts {
        epsilon *= 2;
        if c < 0 {
            epsilon +=1;
        }
    }
    gamma * epsilon
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
        assert_eq!(find_gamma_multiplied_by_epsilon(&input), 198);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 1071734);
    }
}
