use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_04_1.txt").expect("Could not read input file.");
    let (random_numbers, mut boards) = parse_input(input);
    play_bingo(&mut boards, &random_numbers)
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    let input =
        fs::read_to_string("input/year_2021/day_04_1.txt").expect("Could not read input file.");
    let (random_numbers, mut boards) = parse_input(input);
    play_bingo_until_last_board(&mut boards, &random_numbers)
}

fn parse_input(input: String) -> (Vec<i64>, Vec<Board>) {
    let mut lines = input.lines();
    let random_numbers = lines
        .next()
        .expect("Input string is empty.")
        .split(',')
        .map(|s| s.parse::<i64>().expect("Cannot parse random numbers"))
        .collect::<Vec<i64>>();
    let board_rows = lines
        .map(|l| {
            if !l.is_empty() {
                l.split_whitespace()
                    .map(|s| s.parse::<i64>().expect("Cannot parse board row"))
                    .collect::<Vec<i64>>()
            } else {
                vec![]
            }
        })
        .collect::<Vec<Vec<i64>>>();
    let boards = board_rows
        .chunks_exact(6)
        .map(|c| {
            Board::new(
                c[1].clone(),
                c[2].clone(),
                c[3].clone(),
                c[4].clone(),
                c[5].clone(),
            )
        })
        .collect::<Vec<Board>>();

    (random_numbers, boards)
}

struct Board {
    rows: Vec<Vec<(i64, bool)>>,
}

impl Board {
    fn new(
        row_1: Vec<i64>,
        row_2: Vec<i64>,
        row_3: Vec<i64>,
        row_4: Vec<i64>,
        row_5: Vec<i64>,
    ) -> Self {
        let mut board = Board { rows: vec![] };
        board.rows.push(row_1.iter().map(|&c| (c, false)).collect());
        board.rows.push(row_2.iter().map(|&c| (c, false)).collect());
        board.rows.push(row_3.iter().map(|&c| (c, false)).collect());
        board.rows.push(row_4.iter().map(|&c| (c, false)).collect());
        board.rows.push(row_5.iter().map(|&c| (c, false)).collect());
        board
    }

    fn mark_number(&mut self, num: i64) {
        self.rows.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|c| {
                if c.0 == num {
                    c.1 = true;
                }
            })
        });
    }

    fn find_winning_numbers(&self) -> Option<Vec<i64>> {
        // check rows
        for row in &self.rows {
            if row.iter().all(|r| r.1) {
                return Some(row.iter().map(|r| r.0).collect());
            }
        }
        // check columns
        for i in 0..5 {
            let col = self.rows.iter().map(|row| row[i]).collect::<Vec<_>>();
            if col.iter().all(|c| c.1) {
                return Some(col.iter().map(|c| c.0).collect());
            }
        }
        None
    }

    fn sum_unmarked_numbers(&self) -> i64 {
        self.rows
            .iter()
            .flatten()
            .fold(0, |acc, elem| if !elem.1 { acc + elem.0 } else { acc })
    }
}

fn play_bingo(boards: &mut Vec<Board>, random_nums: &[i64]) -> i64 {
    for &rn in random_nums {
        mark_numbers(boards, rn);
        if let Some(winning_board) = find_winning_board(boards) {
            let sum = winning_board.sum_unmarked_numbers();
            return sum * rn;
        }
    }
    0
}

fn play_bingo_until_last_board(boards: &mut Vec<Board>, random_nums: &[i64]) -> i64 {
    for &rn in random_nums {
        mark_numbers(boards, rn);
        let winning_boards = drain_filter_winning_boards(boards);
        if boards.is_empty() {
            let sum = winning_boards[0].sum_unmarked_numbers();
            return sum * rn;
        }
    }
    0
}

fn mark_numbers(boards: &mut Vec<Board>, num: i64) {
    boards.iter_mut().for_each(|b| b.mark_number(num));
}

fn find_winning_board(boards: &[Board]) -> Option<&Board> {
    boards.iter().find(|b| b.find_winning_numbers().is_some())
}

fn drain_filter_winning_boards(boards: &mut Vec<Board>) -> Vec<Board> {
    let mut winning_boards = vec![];
    let mut i = 0;
    while i < boards.len() {
        if boards[i].find_winning_numbers().is_some() {
            winning_boards.push(boards.remove(i));
        } else {
            i += 1;
        }
    }
    winning_boards
}

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn check_example_input_part_1() {
        let random_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards = vec![
            Board::new(
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ),
            Board::new(
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ),
            Board::new(
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ),
        ];
        assert_eq!(play_bingo(&mut boards, &random_numbers), 4512);
    }

    #[test]
    fn check_answer_part_1() {
        assert_eq!(part_1(), 60368);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn check_example_input() {
        let random_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards = vec![
            Board::new(
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ),
            Board::new(
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ),
            Board::new(
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ),
        ];
        assert_eq!(
            play_bingo_until_last_board(&mut boards, &random_numbers),
            1924
        );
    }

    #[test]
    fn check_answer() {
        assert_eq!(part_2(), 17435);
    }
}
