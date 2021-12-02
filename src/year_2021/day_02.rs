use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn part_1() {
    let file = File::open("input/year_2021/day_02_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let steps: Vec<Step> = reader
        .lines()
        .map(|line| line.expect("Could not read line."))
        .map(|line| parse_step(&line).expect("Could not parse line"))
        .collect();
    let mut pos = Position::default();
    pos.process_all(&steps);
    let result = pos.x * pos.y;
    println!("Final position: {}, {}", pos.x, pos.y);
    println!("Day 02/1 result: {}", result);
}

#[allow(dead_code)]
pub fn part_2() {
    let file = File::open("input/year_2021/day_02_1.txt").expect("Could not open input file.");
    let reader = BufReader::new(file);
    let steps: Vec<Step> = reader
        .lines()
        .map(|line| line.expect("Could not read line."))
        .map(|line| parse_step(&line).expect("Could not parse line"))
        .collect();
    let mut sub = Submarine::default();
    sub.process_all(&steps);
    let result = sub.pos.x * sub.pos.y;
    println!("Final position: {}, {}", sub.pos.x, sub.pos.y);
    println!("Day 02/2 result: {}", result);
}

enum Step {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(Debug, Default, PartialEq)]
struct Position {
    x: i64,
    y: i64,
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

#[derive(Debug, Default, PartialEq)]
struct Submarine {
    pos: Position,
    aim: i64,
}

impl Submarine {
    fn process(&mut self, step: &Step) {
        match step {
            Step::Forward(x) => { self.pos.x += x; self.pos.y += self.aim * x }
            Step::Down(delta_aim) => self.aim += delta_aim,
            Step::Up(delta_aim) => self.aim -= delta_aim,
        }
    }

    fn process_all(&mut self, steps: &[Step]) {
        for step in steps {
            self.process(step);
        }
    }
}

fn parse_step(line: &str) -> Result<Step, &str> {
    if let [step_type, value] = line.split(' ').collect::<Vec<&str>>()[..] {
        let value = value.parse::<i64>().expect("Can not parse number");
        match step_type {
            "forward" => Ok(Step::Forward(value)),
            "down" => Ok(Step::Down(value)),
            "up" => Ok(Step::Up(value)),
            _ => Err("Incorrect step type"),
        }
    } else {
        Err("Line format must be 'step_type value'")
    }
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
    fn check_example_input_part_1() {
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

    #[test]
    fn check_example_input_part_2() {
        let mut sub = Submarine::default();
        let steps = vec![
            Step::Forward(5),
            Step::Down(5),
            Step::Forward(8),
            Step::Up(3),
            Step::Down(8),
            Step::Forward(2),
        ];
        sub.process_all(&steps);
        assert_eq!(sub.pos, Position { x: 15, y: 60 });
    }
}
