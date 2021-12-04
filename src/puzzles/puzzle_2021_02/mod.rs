mod engine;
mod instruction;
mod submarine;

use crate::input::PuzzleInput;

use self::engine::Engine;
use self::instruction::Instruction;
use self::submarine::{Position, Submarine};
use std::error::Error;

struct Engine1;

impl Engine for Engine1 {
    fn compute_position((x, y, aim): Position, instruction: Instruction) -> Position {
        match instruction {
            Instruction::Forward(value) => (x + value, y, aim),
            Instruction::Up(value) => (x, y - value, aim),
            Instruction::Down(value) => (x, y + value, aim),
        }
    }
}

struct Engine2;

impl Engine for Engine2 {
    fn compute_position((x, y, aim): Position, instruction: Instruction) -> Position {
        match instruction {
            Instruction::Forward(value) => (x + value, y + aim * value, aim),
            Instruction::Up(value) => (x, y, aim - value),
            Instruction::Down(value) => (x, y, aim + value),
        }
    }
}

pub fn solve<E: Engine>(input: PuzzleInput) -> Result<i64, Box<dyn Error>> {
    let instructions: Vec<Instruction> = input.lines()?;
    let mut submarine: Submarine<E> = Submarine::new();

    for instruction in instructions {
        submarine.move_position(instruction)
    }

    Ok(submarine.x * submarine.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT_FILE: &str = "puzzles/2021/puzzle_02.input";
    const EXAMPLE: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn example1() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve::<Engine1>(input).unwrap();

        assert_eq!(solution, 150);
    }

    #[test]
    fn example2() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve::<Engine2>(input).unwrap();

        assert_eq!(solution, 900);
    }

    #[test]
    fn part1() {
        let content = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let input = PuzzleInput::new(content);
        let solution = solve::<Engine1>(input).unwrap();

        assert_eq!(solution, 1868935);
    }

    #[test]
    fn part2() {
        let content = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let input = PuzzleInput::new(content);
        let solution = solve::<Engine2>(input).unwrap();

        assert_eq!(solution, 1965970888);
    }
}
