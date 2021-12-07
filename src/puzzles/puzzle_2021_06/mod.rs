use anyhow::Error;
use itertools::Itertools;

use crate::{input::PuzzleInput, util};

pub fn solve(input: PuzzleInput, total_days: usize) -> Result<i64, Error> {
    let fishes = input.lines::<String>()?;

    let fishes = fishes
        .iter()
        .flat_map(|l| l.split(',').map(|s| str::parse::<i64>(s).unwrap()))
        .collect_vec();

    let mut counters = util::frequencies(fishes).collect_vec();

    for i in 0..total_days {
        let mut new_entries: Vec<(i64, i64)> = Vec::new();

        match counters.iter().find(|(day, _)| *day == i as i64) {
            None => continue,
            Some((_, count)) => {
                new_entries.push((i as i64 + 9, *count)); // New fishes
                new_entries.push((i as i64 + 7, *count)); // Reset fishes
            }
        };

        for (day, count) in new_entries {
            if let Some((_, b)) = counters.iter_mut().find(|(d, _)| *d == day) {
                *b += count;
            } else {
                counters.push((day, count));
            }
        }

        // Prune old values
        counters = counters
            .into_iter()
            .filter(|(day, _)| *day > i as i64)
            .collect_vec();
    }

    let total: i64 = counters.iter().map(|(_, count)| *count).sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    const INPUT_FILE: &str = "inputs/2021/puzzle_06.input";
    const EXAMPLE: &str = r#"3,4,3,1,2"#;

    #[test]
    fn solve_example_1() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve(input, 80).unwrap();

        assert_eq!(solution, 5934);
    }

    #[test]
    fn solve_input_1() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve(input, 80).unwrap();

        assert_eq!(solution, 393019);
    }

    #[test]
    fn solve_example_2() {
        let input = PuzzleInput::new(EXAMPLE);
        let solution = solve(input, 256).unwrap();

        assert_eq!(solution, 26984457539);
    }

    #[test]
    fn solve_input_2() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let input = PuzzleInput::new(content);
        let solution = solve(input, 256).unwrap();

        assert_eq!(solution, 1757714216975);
    }
}
