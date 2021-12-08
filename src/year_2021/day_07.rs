use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_07_1.txt").expect("Could not read input file.");
    let crabs = parse_input(&input);
    part_1::find_min_fuel_required(&crabs)
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_07_1.txt").expect("Could not read input file.");
    let crabs = parse_input(&input);
    part_2::find_min_fuel_required(&crabs)
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

mod part_2 {

    pub fn find_min_fuel_required(crabs: &[i64]) -> i64 {
        (0..=crabs.len() as i64)
            .map(|crab| total_fuel_required(crabs, crab))
            .min()
            .unwrap_or(0)
    }

    fn total_fuel_required(crabs: &[i64], destination: i64) -> i64 {
        crabs
            .iter()
            .fold(0, |acc, elem| fuel_required(*elem, destination) + acc)
    }

    fn fuel_required(crab: i64, destination: i64) -> i64 {
        (0..=i64::abs(destination - crab)).sum::<i64>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn check_fuel_required() {
            assert_eq!(fuel_required(16, 5), 66);
        }

        #[test]
        fn check_total_fuel_required() {
            let crabs: Vec<i64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            assert_eq!(total_fuel_required(&crabs, 5), 168);
            assert_eq!(total_fuel_required(&crabs, 2), 206);
        }

        #[test]
        fn check_example_input() {
            let crabs: Vec<i64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            assert_eq!(find_min_fuel_required(&crabs), 168);
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

    #[test]
    fn check_answer_part_2() {
        assert_eq!(part_2(), 98257206);
    }
}
