use std::fs;

#[allow(dead_code)]
pub fn part_1() {
    let location = "input/year_2019/day_2_1.txt";
    let mut input: Vec<usize> = fs::read_to_string(location)
        .expect("Cannot read input file")
        .split(",")
        .map(|s| s.to_string())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    input[1] = 12;
    input[2] = 2;
    let result = process_intcode(input);
    println!("IntCode at position 0: {}", result[0]);
}

#[allow(dead_code)]
pub fn part_2() {
    let location = "input/year_2019/day_2_1.txt";
    let input: Vec<usize> = fs::read_to_string(location)
        .expect("Cannot read input file")
        .split(",")
        .map(|s| s.to_string())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_loop = input.clone();
            input_loop[1] = noun;
            input_loop[2] = verb;
            let result = process_intcode(input_loop)[0];
            if result == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                break;
            }
        }
    }
}

fn process_intcode(input: Vec<usize>) -> Vec<usize> {
    let mut result = input.clone();
    let mut index = 0;
    while index < result.len() {
        if result[index] == 1 {
            // process Add
            let src1 = result[index+1];
            let src2 = result[index+2];
            let dest = result[index+3];
            result[dest] = result[src1] + result[src2];
        } else if result[index] == 2 {
            // process Multiply
            let src1 = result[index+1];
            let src2 = result[index+2];
            let dest = result[index+3];
            result[dest] = result[src1] * result[src2];
        } else if result[index] == 99 {
            // process Finish
            break;
        } else {
            panic!();
        }
        index = index + 4;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_1() {
        let input = vec!(
            1, 0, 0, 0,
            99);
        let output = vec!(
            2, 0, 0, 0,
            99);
        assert_eq!(process_intcode(input), output);
    }

    #[test]
    fn example_1_2() {
        let input = vec!(
            2, 3, 0, 3,
            99);
        let output = vec!(
            2, 3, 0, 6,
            99);
        assert_eq!(process_intcode(input), output);
    }

    #[test]
    fn example_1_3() {
        let input = vec!(
            2, 4, 4, 5,
            99,
            0);
        let output = vec!(
            2, 4, 4, 5,
            99,
            9801);
        assert_eq!(process_intcode(input), output);
    }

    #[test]
    fn example_1_4() {
        let input = vec!(
            1, 1, 1, 4,
            99,
            5, 6, 0, 99);
        let output = vec!(
            30, 1, 1, 4,
            2, 5, 6, 0,
            99);
        assert_eq!(process_intcode(input), output);
    }

    #[test]
    fn example_1_5() {
        let input = vec!(
            1, 9, 10, 3,
            2, 3, 11, 0,
            99,
            30, 40, 50);
        let output = vec!(
            3500, 9, 10, 70,
            2, 3, 11, 0,
            99,
            30, 40, 50);
        assert_eq!(process_intcode(input), output);
    }
}
