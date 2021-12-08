use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_07_1.txt").expect("Could not read input file.");
    let crabs = parse_input(&input);
    part_1::find_min_fuel_required(&crabs)
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().expect("Can not parse input"))
        .collect()
}

mod part_1 {

    pub fn find_min_fuel_required(crabs: &[i64]) -> i64 {
        crabs
            .iter()
            .map(|crab| {
                crabs
                    .iter()
                    .fold(0, |acc, elem| i64::abs(crab - elem) + acc)
            })
            .min()
            .expect("Should always be 'Some' as fold will always return at least 1 element")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn check_example_input() {
            let crabs: Vec<i64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            assert_eq!(find_min_fuel_required(&crabs), 37);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let expected_crabs: Vec<i64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(parse_input(input), expected_crabs);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 347509);
    }
}
