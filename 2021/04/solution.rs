use std::fs;
use std::iter::Iterator;

const BOARD_SIZE: usize = 5;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Unmarked(u32),
    Marked(u32),
}

type Cells = [Cell; BOARD_SIZE.pow(2)];

#[derive(Clone, Debug)]
struct Board(Cells);

impl Board {
    fn mark_number(&mut self, number: u32) {
        for i in 0..self.0.len() {
            match self.0[i] {
                Cell::Unmarked(x) if x == number => {
                    self.0[i] = Cell::Marked(x);
                    return;
                }
                Cell::Marked(x) if x == number => return,
                _ => {}
            }
        }
    }

    fn is_completed(&self) -> bool {
        (0..BOARD_SIZE)
            .map(|i| {
                (0..BOARD_SIZE)
                    .map(|j| self.0[BOARD_SIZE * i + j])
                    .all(|cell| match cell {
                        Cell::Marked(_) => true,
                        Cell::Unmarked(_) => false,
                    })
                    || (0..BOARD_SIZE)
                        .map(|j| self.0[BOARD_SIZE * j + i])
                        .all(|cell| match cell {
                            Cell::Marked(_) => true,
                            Cell::Unmarked(_) => false,
                        })
            })
            .any(|col_is_marked| col_is_marked)
    }

    fn score(&self, multiplier: u32) -> u32 {
        self.0.iter().fold(0, |acc, cell| {
            if let Cell::Unmarked(x) = cell {
                acc + x
            } else {
                acc
            }
        }) * multiplier
    }
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn parse_input() -> (Vec<u32>, Vec<Board>) {
    let input = fs::read_to_string("input.txt").unwrap();
    let (numbers_raw, boards_raw) = input.split_once("\n\n").unwrap();
    let numbers: Vec<u32> = numbers_raw.split(",").map(|x| x.parse().unwrap()).collect();
    let boards: Vec<Board> = boards_raw
        .split("\n\n")
        .map(|lines| {
            let mut cells: Cells = [Cell::Unmarked(0); BOARD_SIZE.pow(2)];
            lines
                .split_whitespace()
                .enumerate()
                .for_each(|(i, x)| cells[i] = Cell::Unmarked(x.parse().unwrap()));
            Board(cells)
        })
        .collect();
    (numbers, boards)
}

fn part1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let numbers = &input.0;
    let boards: Vec<Board> = input.1.iter().map(|board| board.clone()).collect();
    numbers
        .iter()
        .scan(boards, |acc_boards, &number| {
            let winning_score = acc_boards
                .iter_mut()
                .map(|board| {
                    board.mark_number(number);
                    board
                })
                .find(|board| board.is_completed())
                .and_then(|board| Some(board.score(number)));
            Some(winning_score)
        })
        .find_map(|winning_score| winning_score)
        .unwrap()
}

fn part2(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let numbers = &input.0;
    let boards: Vec<Board> = input.1.iter().map(|board| board.clone()).collect();
    numbers
        .iter()
        .scan(boards, |acc_boards, &number| {
            if acc_boards.is_empty() {
                return None;
            }
            acc_boards
                .iter_mut()
                .for_each(|board| board.mark_number(number));
            let (completed_boards, remaining_boards): (Vec<Board>, Vec<Board>) = acc_boards
                .into_iter()
                .map(|board| board.clone())
                .partition(|board| board.is_completed());
            *acc_boards = remaining_boards;
            let completed_scores: Vec<_> = completed_boards
                .iter()
                .map(|board| board.score(number))
                .collect();
            Some(completed_scores)
        })
        .flatten()
        .last()
        .unwrap()
}
