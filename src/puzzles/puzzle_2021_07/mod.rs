use anyhow::{anyhow, Error};
use itertools::Itertools;

use crate::{input::PuzzleInput, util};

fn parse_input(input: &PuzzleInput) -> Result<Vec<i64>, Error> {
    Ok(input
        .lines::<String>()?
        .iter()
        .flat_map(|l| l.split(',').map(|n| n.parse::<i64>()))
        .try_collect()?)
}

pub fn solve_part1(input: PuzzleInput) -> Result<i64, Error> {
    let positions = parse_input(&input)?;
    let max_position = *positions.iter().max().ok_or(anyhow!("cannot find max"))?;

    let solution: i64 = (0..max_position)
        .map(|target| {
            positions
                .iter()
                .map(|position| i64::abs(target - position))
                .sum()
        })
        .min()
        .ok_or(anyhow!("cannot find min"))?;

    Ok(solution)
}

pub fn solve_part2(input: PuzzleInput) -> Result<i64, Error> {
    let positions = parse_input(&input)?;
    let max_position = *positions.iter().max().ok_or(anyhow!("cannot find max"))?;

    let solution: i64 = (0..max_position)
        .map(|target| {
            positions
                .iter()
                .map(|position| i64::abs(target - position))
                .map(util::compute_geometric_sum)
                .sum()
        })
        .min()
        .ok_or(anyhow!("cannot find min"))?;

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    const INPUT_FILE: &str = "inputs/2021/puzzle_07.input";
    const EXAMPLE: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn solve_example_1() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 37);
    }

    #[test]
    fn solve_input_1() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 336131);
    }

    #[test]
    fn solve_example_2() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 168);
    }

    #[test]
    fn solve_input_2() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 92676646);
    }
}
