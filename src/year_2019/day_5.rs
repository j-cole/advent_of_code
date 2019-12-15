use crate::year_2019::intcode::Intcode;
use std::fs;

pub fn part_1() {
    let location = "input/year_2019/day_5_1.txt";
    let input: Vec<i64> = fs::read_to_string(location)
        .expect("Cannot read input file")
        .split(",")
        .map(|s| s.to_string())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();
    let mut intcode = Intcode::new(input).expect("Input cannot be intialized");
    intcode.reset().run();
    // 6731945
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_5_1() {
        let input = vec![1002, 4, 3, 4, 33];
        let output = vec![1002, 4, 3, 4, 99];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }

    #[test]
    fn example_5_2() {
        let input = vec![1101, 100, -1, 4, 0];
        let output = vec![1101, 100, -1, 4, 99];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }
}
