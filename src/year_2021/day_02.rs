use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, PartialEq)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn process(&mut self, step: &Step) {
        match step {
            Step::Forward(x) => self.x += x,
            Step::Down(y) => self.y += y,
            Step::Up(y) => self.y -= y,
        }
    }

    fn process_all(&mut self, steps: &[Step]) {
        for step in steps {
            self.process(step);
        }
    }
}

enum Step {
    Forward(u64),
    Down(u64),
    Up(u64),
}

fn parse_step(line: &str) -> Result<Step, &str> {
    if line.starts_with("forward ") {
        Ok(Step::Forward(
            line[8..].parse::<u64>().expect("Can not parse number"),
        ))
    } else if line.starts_with("down ") {
        Ok(Step::Down(
            line[5..].parse::<u64>().expect("Can not parse number"),
        ))
    } else if line.starts_with("up ") {
        Ok(Step::Up(
            line[3..].parse::<u64>().expect("Can not parse number"),
        ))
    } else {
        Err("Can not parse line")
    }
}

#[allow(dead_code)]
pub fn part_1() {
    let file = File::open("input/year_2021/day_02_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let steps: Vec<Step> = reader
        .lines()
        .map(|line| line.expect("Could not read line."))
        .map(|line| parse_step(&line).expect("Could not parse line"))
        .collect();
    let mut pos = Position { x: 0, y: 0 };
    pos.process_all(&steps);
    let result = pos.x * pos.y;
    println!("Final position: {}, {}", pos.x, pos.y);
    println!("Day 02/1 result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_increases_x() {
        let mut pos = Position::default();
        let step = Step::Forward(10);
        pos.process(&step);
        assert_eq!(pos, Position { x: 10, y: 0 });
    }

    #[test]
    fn down_increases_y() {
        let mut pos = Position::default();
        let step = Step::Down(10);
        pos.process(&step);
        assert_eq!(pos, Position { x: 0, y: 10 });
    }

    #[test]
    fn up_decreases_y() {
        let mut pos = Position { x: 0, y: 20 };
        let step = Step::Up(5);
        pos.process(&step);
        assert_eq!(pos, Position { x: 0, y: 15 });
    }

    #[test]
    fn check_example_input_1() {
        let mut pos = Position::default();
        let steps = vec![
            Step::Forward(5),
            Step::Down(5),
            Step::Forward(8),
            Step::Up(3),
            Step::Down(8),
            Step::Forward(2),
        ];
        pos.process_all(&steps);
        assert_eq!(pos, Position { x: 15, y: 10 });
    }
}
