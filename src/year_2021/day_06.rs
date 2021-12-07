use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_06_1.txt").expect("Could not read input file.");
    let fish = parse_input(&input);
    let mut simulation = LanternfishSimulation { fish };
    simulation.run(80);
    simulation.count() as i64
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().expect("Can not parse input"))
        .collect()
}

struct LanternfishSimulation {
    fish: Vec<i64>,
}

impl LanternfishSimulation {

    fn step(&mut self) {
        let mut count_new_fish = 0;
        for f in &mut self.fish {
            if *f == 0 {
                *f = 6;
                count_new_fish += 1;
            } else {
                *f -= 1;
            }
        }
        self.fish.extend(vec![8; count_new_fish]);
    }

    fn run(&mut self, num_steps: i64) {
        for _ in 0..num_steps {
            self.step();
        }
    }

    fn count(&self) -> usize {
        self.fish.len()
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::*;

        #[test]
        fn parse_example_input() {
            let input = "3,4,3,1,2";
            let expected_output: Vec<i64> = vec![3, 4, 3, 1, 2];
            assert_eq!(parse_input(input), expected_output);
        }

        #[test]
        fn check_example_input() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation { fish: input };
            simulation.step();
            assert_eq!(simulation.fish, vec![2, 3, 2, 0, 1]);
            simulation.step();
            assert_eq!(simulation.fish, vec![1, 2, 1, 6, 0, 8]);
        }

        #[test]
        fn check_answer() {
            assert_eq!(part_1(), 374927);
        }
    }
}
