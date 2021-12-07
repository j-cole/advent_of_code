use regex::Regex;
use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_05_1.txt").expect("Could not read input file.");
    let lines = parse_input(&input);
    count_dangerous_points(&lines)
}

fn parse_input(input: &str) -> Vec<Line> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").expect("Could not create regex.");
    input
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|caps| {
            let values: Vec<i64> = caps
                .iter()
                .flatten()
                .skip(1)
                .map(|regex_match| {
                    regex_match
                        .as_str()
                        .parse::<i64>()
                        .expect("Could not parse line value.")
                })
                .collect();
            Line {
                p1: Point {
                    x: values[0],
                    y: values[1],
                },
                p2: Point {
                    x: values[2],
                    y: values[3],
                },
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Line {
    p1: Point,
    p2: Point,
}

fn count_dangerous_points(lines: &[Line]) -> i64 {
    let floor_size = 1000;
    lines
        .iter()
        .filter(|l| l.p1.x == l.p2.x || l.p1.y == l.p2.y)
        .fold(vec![vec![0; floor_size]; floor_size], |mut acc, elem| {
            let x1 = elem.p1.x;
            let x2 = elem.p2.x;
            let y1 = elem.p1.y;
            let y2 = elem.p2.y;
            let range_y = (i64::min(y1, y2)..=i64::max(y1, y2)).collect::<Vec<i64>>();
            let range_x = (i64::min(x1, x2)..=i64::max(x1, x2)).collect::<Vec<i64>>();
            for y in &range_y {
                for x in &range_x {
                    acc[*y as usize][*x as usize] += 1;
                }
            }
            acc
        })
        .into_iter()
        .flatten()
        .filter(|f| 1 < *f)
        .count() as i64
}

#[cfg(test)]
mod part_1 {
    use super::*;

    fn create_example_lines() -> Vec<Line> {
        vec![
            Line {
                p1: Point { x: 0, y: 9 },
                p2: Point { x: 5, y: 9 },
            },
            Line {
                p1: Point { x: 8, y: 0 },
                p2: Point { x: 0, y: 8 },
            },
            Line {
                p1: Point { x: 9, y: 4 },
                p2: Point { x: 3, y: 4 },
            },
            Line {
                p1: Point { x: 2, y: 2 },
                p2: Point { x: 2, y: 1 },
            },
            Line {
                p1: Point { x: 7, y: 0 },
                p2: Point { x: 7, y: 4 },
            },
            Line {
                p1: Point { x: 6, y: 4 },
                p2: Point { x: 2, y: 0 },
            },
            Line {
                p1: Point { x: 0, y: 9 },
                p2: Point { x: 2, y: 9 },
            },
            Line {
                p1: Point { x: 3, y: 4 },
                p2: Point { x: 1, y: 4 },
            },
            Line {
                p1: Point { x: 0, y: 0 },
                p2: Point { x: 8, y: 8 },
            },
            Line {
                p1: Point { x: 5, y: 5 },
                p2: Point { x: 8, y: 2 },
            },
        ]
    }

    #[test]
    fn parse_example_input() {
        let input = "\
            0,9 -> 5,9\n\
            8,0 -> 0,8\n\
            9,4 -> 3,4\n\
            2,2 -> 2,1\n\
            7,0 -> 7,4\n\
            6,4 -> 2,0\n\
            0,9 -> 2,9\n\
            3,4 -> 1,4\n\
            0,0 -> 8,8\n\
            5,5 -> 8,2\n\
        ";
        let expected_output: Vec<Line> = create_example_lines();
        assert_eq!(parse_input(input), expected_output);
    }

    #[test]
    fn check_example_input() {
        let input: Vec<Line> = create_example_lines();
        assert_eq!(count_dangerous_points(&input), 5);
    }

    #[test]
    fn check_answer() {
        assert_eq!(part_1(), 5632);
    }
}
