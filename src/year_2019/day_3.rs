use core::cmp;
use std::fs;

#[allow(dead_code)]
pub fn part_1() {
    let location = "input/year_2019/day_3_1.txt";
    let input: String = fs::read_to_string(location).expect("Cannot read input file");
    let mut wires = input.lines();
    let points1 = parse_points(wires.next().expect("Missing input line 1"));
    let points2 = parse_points(wires.next().expect("Missing input line 2"));
    let distance = min_manhattan_distance(&points1, &points2);
    println!("The manhattan distance is: {}", distance);
}

#[allow(dead_code)]
pub fn part_2() {
    let location = "input/year_2019/day_3_1.txt";
    let input: String = fs::read_to_string(location).expect("Cannot read input file");
    let mut wires = input.lines();
    let points1 = parse_points(wires.next().expect("Missing input line 1"));
    let points2 = parse_points(wires.next().expect("Missing input line 2"));
    let steps = min_steps(&points1, &points2);
    println!("The steps are: {}", steps);
}

fn parse_points(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];
    let mut current = Point { x: 0, y: 0 };
    input.split(',').map(Direction::from).for_each(|d| {
        current = Point::new(&current, &d);
        points.push(current);
    });
    points
}

struct Intersection {
    point: Point,
    steps1: u64,
    steps2: u64,
}

fn find_intersections(points1: &[Point], points2: &[Point]) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = vec![];
    let mut steps1 = 0;
    for line1 in points1.windows(2) {
        let mut steps2 = 0;
        for line2 in points2.windows(2) {
            if let Some(point) = intersect(line1, line2) {
                let intersection_steps1 = steps1 + steps(&line1[0], &point);
                let intersection_steps2 = steps2 + steps(&line2[0], &point);
                intersections.push(Intersection {
                    point,
                    steps1: intersection_steps1,
                    steps2: intersection_steps2,
                });
            }
            steps2 += steps(&line2[0], &line2[1]);
        }
        steps1 += steps(&line1[0], &line1[1]);
    }
    intersections
}

fn steps(point1: &Point, point2: &Point) -> u64 {
    ((point2.x - point1.x).abs() + (point2.y - point1.y).abs()) as u64
}

fn min_manhattan_distance(wire1: &[Point], wire2: &[Point]) -> u64 {
    let intersections = find_intersections(wire1, wire2);
    intersections
        .iter()
        .filter(|i| i.point.x != 0 && i.point.y != 0)
        .map(|i| (i.point.x.abs() + i.point.y.abs()) as u64)
        .min()
        .unwrap_or(0)
}

fn min_steps(wire1: &[Point], wire2: &[Point]) -> u64 {
    let intersections = find_intersections(wire1, wire2);
    intersections
        .iter()
        .filter(|i| i.point.x != 0 && i.point.y != 0)
        .map(|i| i.steps1 + i.steps2)
        .min()
        .unwrap_or(0)
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
        let distance = chars
            .collect::<String>()
            .parse::<i64>()
            .expect("Cannot parse distance");
        match direction {
            'U' => Direction::Up(distance),
            'R' => Direction::Right(distance),
            'D' => Direction::Down(distance),
            'L' => Direction::Left(distance),
            _ => panic!("Cannot parse direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(point: &Point, dir: &Direction) -> Self {
        match dir {
            Direction::Up(distance) => Point {
                x: point.x,
                y: point.y + distance,
            },
            Direction::Right(distance) => Point {
                x: point.x + distance,
                y: point.y,
            },
            Direction::Down(distance) => Point {
                x: point.x,
                y: point.y - distance,
            },
            Direction::Left(distance) => Point {
                x: point.x - distance,
                y: point.y,
            },
        }
    }
}

fn intersect(line1: &[Point], line2: &[Point]) -> Option<Point> {
    if is_horizontal(&line1[0], &line1[1]) && is_vertical(&line2[0], &line2[1]) {
        let left = cmp::min(line1[0].x, line1[1].x);
        let right = cmp::max(line1[0].x, line1[1].x);
        let bottom = cmp::min(line2[0].y, line2[1].y);
        let top = cmp::max(line2[0].y, line2[1].y);
        if (left..=right).contains(&line2[0].x) && (bottom..=top).contains(&line1[0].y) {
            Some(Point {
                x: line2[0].x,
                y: line1[0].y,
            })
        } else {
            None
        }
    } else if is_vertical(&line1[0], &line1[1]) && is_horizontal(&line2[0], &line2[1]) {
        let left = cmp::min(line2[0].x, line2[1].x);
        let right = cmp::max(line2[0].x, line2[1].x);
        let bottom = cmp::min(line1[0].y, line1[1].y);
        let top = cmp::max(line1[0].y, line1[1].y);
        if (left..=right).contains(&line1[0].x) && (bottom..=top).contains(&line2[0].y) {
            Some(Point {
                x: line1[0].x,
                y: line2[0].y,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn is_vertical(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y != p2.y
}

fn is_horizontal(p1: &Point, p2: &Point) -> bool {
    p1.x != p2.x && p1.y == p2.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_1() {
        let points1 = parse_points("R8,U5,L5,D3");
        let points2 = parse_points("U7,R6,D4,L4");
        assert_eq!(min_manhattan_distance(&points1, &points2), 6);
    }

    #[test]
    fn example_3_2() {
        let points1 = parse_points("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let points2 = parse_points("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(min_manhattan_distance(&points1, &points2), 159);
    }

    #[test]
    fn example_3_3() {
        let points1 = parse_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let points2 = parse_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(min_manhattan_distance(&points1, &points2), 135);
    }

    #[test]
    fn example_3_4() {
        let points1 = parse_points("R8,U5,L5,D3");
        let points2 = parse_points("U7,R6,D4,L4");
        assert_eq!(min_steps(&points1, &points2), 30);
    }

    #[test]
    fn example_3_5() {
        let points1 = parse_points("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let points2 = parse_points("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(min_steps(&points1, &points2), 610);
    }

    #[test]
    fn example_3_6() {
        let points1 = parse_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let points2 = parse_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(min_steps(&points1, &points2), 410);
    }
}
