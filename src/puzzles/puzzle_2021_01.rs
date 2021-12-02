use itertools::Itertools;
use std::error::Error;

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let solution = input
        .lines()
        .map(|n| n.parse::<i64>().expect("invalid value"))
        .tuple_windows()
        .filter(|(a, b)| b - a > 0)
        .count();

    Ok(solution as i64)
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let solution = input
        .lines()
        .map(|n| n.parse::<i64>().expect("invalid value"))
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b - a > 0)
        .count();

    Ok(solution as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT_FILE: &str = "puzzles/2021/puzzle_01.input";
    const EXAMPLE: &str = r"199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn example1() {
        let solution = solve_part1(EXAMPLE).unwrap();

        assert_eq!(solution, 7);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let solution = solve_part1(&input).unwrap();

        assert_eq!(solution, 1482);
    }

    #[test]
    fn example2() {
        let solution = solve_part2(EXAMPLE).unwrap();

        assert_eq!(solution, 5);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string(INPUT_FILE).expect("error reading file");
        let solution = solve_part2(&input).unwrap();

        assert_eq!(solution, 1518);
    }
}
