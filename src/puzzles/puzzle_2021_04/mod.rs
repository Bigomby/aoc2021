mod board;

use self::board::Board;
use crate::input::PuzzleInput;
use anyhow::Error;
use itertools::Itertools;

fn parse_numbers(number_entries: &str) -> Result<Vec<i64>, Error> {
    Ok(number_entries
        .split(',')
        .map(|n| n.parse::<i64>())
        .try_collect()?)
}

fn parse_boards(boards_entries: &[&str]) -> Vec<Board> {
    boards_entries
        .iter()
        .map(|board| board.split('\n').collect_vec())
        .map(|input| Board::new(&input))
        .collect_vec()
}

fn compute_winners(boards: &mut [Board], numbers: &[i64]) -> Vec<(usize, i64)> {
    let mut winners = Vec::new();

    for number in numbers {
        let mut round_winners = boards
            .iter_mut()
            .enumerate()
            .filter(|(_, board)| !board.has_won())
            .filter_map(move |(index, board)| {
                board.mark_number(*number);
                if board.has_won() {
                    return Some(index);
                }

                None
            })
            .map(|index| (index, *number))
            .collect_vec();

        if !round_winners.is_empty() {
            winners.append(&mut round_winners);
        }

        if winners.len() >= boards.len() {
            break;
        }
    }

    winners
}

fn parse_input(input: PuzzleInput) -> Result<(Vec<Board>, Vec<i64>), Error> {
    let sections = input.content().split("\n\n").collect_vec();
    let number_entries = sections.get(0).unwrap();
    let boards_entries = sections.get(1..).unwrap();

    let boards = parse_boards(boards_entries);
    let numbers = parse_numbers(number_entries)?;

    Ok((boards, numbers))
}

fn solve(index: usize, winning_number: i64, boards: &[Board]) -> i64 {
    let winner_board = boards.get(index).expect("winner not found");

    winner_board.unmarked_numbers().sum::<i64>() * winning_number
}

pub fn solve_part1(input: PuzzleInput) -> Result<i64, Error> {
    let (mut boards, numbers) = parse_input(input)?;
    let winners = compute_winners(&mut boards, &numbers);

    let (last_winner_index, winning_number) = *winners.first().expect("no one won");
    let solution = solve(last_winner_index, winning_number, &boards);

    Ok(solution)
}

pub fn solve_part2(input: PuzzleInput) -> Result<i64, Error> {
    let (mut boards, numbers) = parse_input(input)?;
    let winners = compute_winners(&mut boards, &numbers);

    let (last_winner_index, winning_number) = *winners.last().expect("no one won");
    let solution = solve(last_winner_index, winning_number, &boards);

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT_FILE: &str = "puzzles/2021/puzzle_04.input";
    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn solve_example_1() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 4512);
    }

    #[test]
    fn solve_input_1() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 2496);
    }

    #[test]
    fn solve_example_2() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 1924);
    }

    #[test]
    fn solve_input_2() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 25925);
    }
}
