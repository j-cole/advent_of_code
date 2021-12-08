use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_06_1.txt").expect("Could not read input file.");
    let fish = parse_input(&input);
    let mut simulation = part_1::LanternfishSimulation::new(&fish);
    simulation.run(80);
    simulation.count()
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_06_1.txt").expect("Could not read input file.");
    let fish = parse_input(&input);
    let mut simulation = part_2::LanternfishSimulation::new(&fish);
    simulation.run(256);
    simulation.count()
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().expect("Can not parse input"))
        .collect()
}

mod part_1 {

    pub struct LanternfishSimulation {
        fish: Vec<i64>,
    }

    impl LanternfishSimulation {
        pub fn new(starting_fish: &[i64]) -> Self {
            LanternfishSimulation {
                fish: starting_fish.to_vec(),
            }
        }

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

        pub fn run(&mut self, num_steps: i64) {
            for _ in 0..num_steps {
                self.step();
            }
        }

        pub fn count(&self) -> i64 {
            self.fish.len() as i64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn check_step() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.step();
            assert_eq!(simulation.fish, vec![2, 3, 2, 0, 1]);
            simulation.step();
            assert_eq!(simulation.fish, vec![1, 2, 1, 6, 0, 8]);
            simulation.step();
            assert_eq!(simulation.fish, vec![0, 1, 0, 5, 6, 7, 8]);
            simulation.step();
            assert_eq!(simulation.fish, vec![6, 0, 6, 4, 5, 6, 7, 8, 8]);
        }

        #[test]
        fn check_example_input_18_steps() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.run(18);
            assert_eq!(simulation.count(), 26);
        }

        #[test]
        fn check_example_input_80_steps() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.run(80);
            assert_eq!(simulation.count(), 5934);
        }
    }
}

mod part_2 {

    pub struct LanternfishSimulation {
        fish: [i64; 9],
    }

    impl LanternfishSimulation {
        pub fn new(starting_fish: &[i64]) -> Self {
            let count_fish = starting_fish.iter().fold([0; 9], |mut acc, elem| {
                if (0..=8).contains(elem) {
                    acc[*elem as usize] += 1;
                }
                acc
            });
            LanternfishSimulation { fish: count_fish }
        }

        fn step(&mut self) {
            let mut next_fish = [0; 9];
            for i in 0..=8 {
                match i {
                    6 => next_fish[6] = self.fish[0] + self.fish[7],
                    8 => next_fish[8] = self.fish[0],
                    i => next_fish[i] = self.fish[i + 1],
                }
            }
            self.fish = next_fish;
        }

        pub fn run(&mut self, num_steps: i64) {
            for _ in 0..num_steps {
                self.step();
            }
        }

        pub fn count(&self) -> i64 {
            self.fish.iter().sum::<i64>() as i64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn check_step() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.step();
            assert_eq!(simulation.fish, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
            simulation.step();
            assert_eq!(simulation.fish, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
            simulation.step();
            assert_eq!(simulation.fish, [2, 1, 0, 0, 0, 1, 1, 1, 1]);
            simulation.step();
            assert_eq!(simulation.fish, [1, 0, 0, 0, 1, 1, 3, 1, 2]);
        }

        #[test]
        fn check_example_input_18_steps() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.run(18);
            assert_eq!(simulation.count(), 26);
        }

        #[test]
        fn check_example_input_80_steps() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.run(80);
            assert_eq!(simulation.count(), 5934);
        }

        #[test]
        fn check_example_input_256_steps() {
            let input: Vec<i64> = vec![3, 4, 3, 1, 2];
            let mut simulation = LanternfishSimulation::new(&input);
            simulation.run(256);
            assert_eq!(simulation.count(), 26984457539);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_input() {
        let input = "3,4,3,1,2";
        let expected_output: Vec<i64> = vec![3, 4, 3, 1, 2];
        assert_eq!(parse_input(input), expected_output);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 374927);
    }

    #[test]
    fn check_answer_part_2() {
        assert_eq!(part_2(), 1687617803407);
    }
}
