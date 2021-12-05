mod line;
mod map;

use anyhow::Error;

use crate::{input::PuzzleInput, puzzle_2021_05::map::Map, util::Line};

pub fn solve_part1(input: PuzzleInput) -> Result<i64, Error> {
    let mut map = Map::new(1000, 1000);

    input
        .lines::<Line>()?
        .into_iter()
        .filter(|line| matches!(line, Line::Horizontal(_, _) | Line::Vertical(_, _)))
        .for_each(|line| map.draw_line(line));

    let solution = map.find_intersection(2).len() as i64;

    Ok(solution)
}

pub fn solve_part2(input: PuzzleInput) -> Result<i64, Error> {
    let mut map = Map::new(1000, 1000);

    input
        .lines::<Line>()?
        .into_iter()
        .for_each(|line| map.draw_line(line));

    let solution = map.find_intersection(2).len() as i64;

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    const INPUT_FILE: &str = "inputs/2021/puzzle_05.input";
    const EXAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn solve_example_1() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 5);
    }

    #[test]
    fn solve_input_1() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part1(input).unwrap();

        assert_eq!(solution, 5835);
    }

    #[test]
    fn solve_example_2() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 12);
    }

    #[test]
    fn solve_input_2() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve_part2(input).unwrap();

        assert_eq!(solution, 17013);
    }
}
