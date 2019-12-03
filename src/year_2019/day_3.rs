use std::fs;

#[allow(dead_code)]
pub fn part_1() {
    let location = "input/year_2019/day_3_1.txt";
    let input: String  = fs::read_to_string(location)
        .expect("Cannot read input file");
    let mut wires = input
        .lines()
        .map(|l| get_wire_from_line(l));
    let wire1 = wires.next().expect("Missing input line 1");
    let wire2 = wires.next().expect("Missing input line 2");
    let distance = manhattan_distance(wire1, wire2);
    println!("The manhattan distance is: {}", distance);
}

fn get_wire_from_line(line: &str) -> Vec<Direction> {
    line.split(",")
        .map(|s| Direction::from(s))
        .collect()
}

enum Direction {
    Up(i64),
    Right(i64),
    Down(i64),
    Left(i64),
}

impl From<&str> for Direction {
    fn from(item: &str) -> Self {
        let mut chars = item.chars();
        let direction: char = chars.next().expect("Cannot parse direction");
        let distance = chars.collect::<String>().parse::<i64>().expect("Cannot parse distance");
        match direction {
            'U' => Direction::Up(distance),
            'R' => Direction::Right(distance),
            'D' => Direction::Down(distance),
            'L' => Direction::Left(distance),
            _ => panic!("Cannot parse direction"),
        }
    }
}

fn manhattan_distance(wire1: Vec<Direction>, wire2: Vec<Direction>) -> i64 {
    let lines1 = get_lines(wire1);
    let lines2 = get_lines(wire2);
    let mut intersections: Vec<Point> = vec!();
    for l1 in &lines1 {
        for l2 in &lines2 {
            if let Some(point) = l1.find_intersection(l2) {
                intersections.push(point);
            }
        }
    }
    intersections
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .expect("No valid intersection")
}

fn get_lines(wire: Vec<Direction>) -> Vec<Line> {
    let mut lines = vec!();
    let mut current = Point { x: 0, y: 0 };
    let mut next = Point { x: 0, y: 0 };
    for w in wire {
        match w {
            Direction::Up(distance) => next.y = next.y + distance,
            Direction::Right(distance) => next.x = next.x + distance,
            Direction::Down(distance) => next.y = next.y - distance,
            Direction::Left(distance) => next.x = next.x - distance,
        }
        let line = Line { p1: current, p2: next };
        lines.push(line);
        current = next;
    }
    lines
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Debug,Clone,Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Line {
    fn find_intersection(&self, other: &Line) -> Option<Point> {
        let x1_range = if self.p1.x <= self.p2.x {
            (self.p1.x ..= self.p2.x)
        } else {
            (self.p2.x ..= self.p1.x)
        };
        let y1_range = if self.p1.y <= self.p2.y {
            (self.p1.y ..= self.p2.y)
        } else {
            (self.p2.y ..= self.p1.y)
        };
        let x2_range = if other.p1.x <= other.p2.x {
            (other.p1.x ..= other.p2.x)
        } else {
            (other.p2.x ..= other.p1.x)
        };
        let y2_range = if other.p1.y <= other.p2.y {
            (other.p1.y ..= other.p2.y)
        } else {
            (other.p2.y ..= other.p1.y)
        };

        if (other.p1.x != 0 || self.p1.y != 0) &&
            x1_range.contains(&other.p1.x) &&
            y2_range.contains(&self.p1.y) {
            Some(Point { x: other.p1.x, y: self.p1.y })
        } else if (other.p1.y != 0 || self.p1.x != 0) &&
            y1_range.contains(&other.p1.y) && x2_range.contains(&self.p1.x) {
            Some(Point { x: self.p1.x, y: other.p1.y })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_1() {
        let wire1 = get_wire_from_line("R8,U5,L5,D3");
        let wire2 = get_wire_from_line("U7,R6,D4,L4");
        assert_eq!(manhattan_distance(wire1, wire2), 6);
    }

    #[test]
    fn example_3_2() {
        let wire1 = get_wire_from_line("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = get_wire_from_line("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(manhattan_distance(wire1, wire2), 159);
    }

    #[test]
    fn example_3_3() {
        let wire1 = get_wire_from_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = get_wire_from_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(manhattan_distance(wire1, wire2), 135);
    }
}
