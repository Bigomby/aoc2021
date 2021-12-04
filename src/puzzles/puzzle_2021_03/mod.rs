use std::error::Error;

use itertools::Itertools;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let entries = input.lines();
    let width = entries.clone().next().unwrap().len();

    let mut gamma = String::new();

    for col in 0..width {
        let mut current_col: Vec<char> = Vec::new();

        for entry in entries.clone() {
            let c = entry.chars().nth(col).unwrap();
            current_col.push(c);
        }

        current_col.sort_unstable();
        let zeroes_count = current_col.iter().take_while(|x| **x == '0').count() as i64;
        let ones_count = entries.clone().count() as i64 - zeroes_count;

        let digit = match zeroes_count - ones_count {
            x if x > 0 => '0',
            x if x < 0 => '1',
            x if x == 0 => 'x',
            _ => unimplemented!(),
        };

        gamma.push(digit);
    }

    let epsilon = gamma.replace('0', "x").replace('1', "0").replace('x', "1");

    let gamma_rate = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_rate = u64::from_str_radix(&epsilon, 2).unwrap();

    Ok((epsilon_rate * gamma_rate) as i64)
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let entries = input.lines();
    let width = entries.clone().next().unwrap().len();

    let mut current_entries = entries.clone().collect_vec();
    for col in 0..width {
        let mut current_col: Vec<char> = Vec::new();

        for entry in current_entries.clone() {
            let c = entry.chars().nth(col).unwrap();
            current_col.push(c);
        }

        current_col.sort_unstable();
        let zeroes_count = current_col.iter().take_while(|x| **x == '0').count() as i64;
        let ones_count = current_entries.clone().len() as i64 - zeroes_count;

        let digit = match zeroes_count - ones_count {
            x if x > 0 => '0',
            x if x <= 0 => '1',
            _ => unimplemented!(),
        };

        current_entries = current_entries
            .into_iter()
            .filter(|x| x.chars().nth(col).unwrap() == digit)
            .collect_vec();
    }

    let oxigen_rating = u64::from_str_radix(current_entries[0], 2).unwrap();

    let mut current_entries = entries.clone().collect_vec();
    for col in 0..width {
        let mut current_col: Vec<char> = Vec::new();

        for entry in current_entries.clone() {
            let c = entry.chars().nth(col).unwrap();
            current_col.push(c);
        }

        current_col.sort_unstable();
        let zeroes_count = current_col.iter().take_while(|x| **x == '0').count() as i64;
        let ones_count = current_entries.clone().len() as i64 - zeroes_count;

        let digit = match zeroes_count - ones_count {
            x if x <= 0 => '0',
            x if x > 0 => '1',
            _ => unimplemented!(),
        };

        if current_entries.len() < 2 {
            break;
        }
        current_entries = current_entries
            .into_iter()
            .filter(|x| x.chars().nth(col).unwrap() == digit)
            .collect_vec();
    }

    let co2_rating = u64::from_str_radix(current_entries[0], 2).unwrap();

    Ok((oxigen_rating * co2_rating) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT_FILE: &str = "puzzles/2021/puzzle_03.input";
    const EXAMPLE_1: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn example1() {
        let solution = solve_part1(EXAMPLE_1).unwrap();

        assert_eq!(solution, 198);
    }

    #[test]
    fn example2() {
        let solution = solve_part2(EXAMPLE_1).unwrap();

        assert_eq!(solution, 230);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let solution = solve_part1(&input).unwrap();

        assert_eq!(solution, 3009600);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let solution = solve_part2(&input).unwrap();

        assert_eq!(solution, 6940518);
    }
}
