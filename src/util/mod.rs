use itertools::Itertools;
use std::{collections::HashMap, error::Error, fmt};

#[allow(dead_code)]
pub fn decode_lines<'a, I, T, F>(input: I, decode: F) -> impl Iterator<Item = T> + 'a
where
    I: Into<&'a str>,
    F: Fn(&'a str) -> T + 'a,
{
    input.into().split('\n').map(decode)
}

#[allow(dead_code)]
pub fn decode_lines_group<'a, I, T, F>(input: I, decode: F) -> impl Iterator<Item = T> + 'a
where
    I: Into<&'a str>,
    F: Fn(&'a str) -> T + 'a,
{
    input.into().split("\n\n").map(decode)
}

#[allow(dead_code)]
pub fn self_cross_product(input: &[i32], dims: usize) -> impl Iterator<Item = Vec<i32>> + '_ {
    (0..dims)
        .map(|_| input.iter().cloned())
        .multi_cartesian_product()
}

#[allow(dead_code)]
pub fn compute_product(xs: &[i32]) -> i64 {
    xs.iter().map(|x| *x as i64).product::<i64>()
}

#[allow(dead_code)]
pub fn check_sum_equals(xs: &[i32], target: i32) -> bool {
    xs.iter().sum::<i32>() == target
}

#[allow(dead_code)]
pub fn take_half(range: (i32, i32), section: char) -> (i32, i32) {
    match section {
        'F' | 'L' => (range.0, range.0 + (range.1 - range.0) / 2),
        'B' | 'R' => (range.1 - (range.1 - range.0) / 2, range.1),
        _ => range,
    }
}

#[allow(dead_code)]
pub fn frequencies(s: &str) -> impl Iterator<Item = (char, usize)> {
    let mut freqs: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        match freqs.get_mut(&c) {
            Some(x) => {
                *x += 1;
            }
            None => {
                freqs.insert(c, 1);
            }
        };
    }

    freqs.into_iter()
}

#[allow(dead_code)]
pub fn concat_vec<T>(a: Vec<T>, b: Vec<T>) -> impl Iterator<Item = T> {
    a.into_iter().chain(b.into_iter())
}

#[derive(Debug)]
struct PuzzleError;

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Puzzle error")
    }
}

impl Error for PuzzleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
