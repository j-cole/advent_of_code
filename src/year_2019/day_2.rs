use crate::year_2019::intcode::Intcode;
use std::fs;

#[allow(dead_code)]
pub fn part_1() {
    let location = "input/year_2019/day_2_1.txt";
    let input: Vec<i64> = fs::read_to_string(location)
        .expect("Cannot read input file")
        .split(",")
        .map(|s| s.to_string())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();
    let mut intcode = Intcode::new(input).expect("Input cannot be intialized");
    intcode.reset();
    intcode.write_noun(12);
    intcode.write_verb(2);
    intcode.run();
    let result = intcode.read_output();
    assert_eq!(result, 10566835);
    println!("IntCode at position 0: {}", result);
}

#[allow(dead_code)]
pub fn part_2() {
    let location = "input/year_2019/day_2_1.txt";
    let input: Vec<i64> = fs::read_to_string(location)
        .expect("Cannot read input file")
        .split(",")
        .map(|s| s.to_string())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();
    let mut intcode = Intcode::new(input).expect("Input cannot be intialized");
    for noun in 0..=99 {
        for verb in 0..=99 {
            intcode.reset();
            intcode.write_noun(noun);
            intcode.write_verb(verb);
            intcode.run();
            if intcode.read_output() == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                assert_eq!(noun * 100 + verb, 2347);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2_1() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }

    #[test]
    fn example_2_2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }

    #[test]
    fn example_2_3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }

    #[test]
    fn example_2_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }

    #[test]
    fn example_2_5() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut intcode = Intcode::new(input).unwrap();
        intcode.reset().run();
        assert_eq!(intcode.dump(), output);
    }
}
