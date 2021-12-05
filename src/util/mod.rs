use itertools::Itertools;
use std::collections::HashMap;

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

pub fn matrix_transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut transposed_matrix = vec![Vec::with_capacity(rows); columns];

    for row in matrix {
        for i in 0..row.len() {
            transposed_matrix[i].push(row[i]);
        }
    }

    transposed_matrix
}

#[derive(Debug)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub enum Line {
    Horizontal(Coords, Coords),
    Vertical(Coords, Coords),
    Diagonal(Coords, Coords),
    Point(Coords),
    Invalid,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Coords>,
}

fn compute_range(start: i64, end: i64) -> Vec<i64> {
    if end - start > 0 {
        (start..=end).collect_vec()
    } else {
        (end..=start).rev().collect_vec()
    }
}

impl From<Line> for Path {
    fn from(line: Line) -> Self {
        let points = match line {
            Line::Invalid => vec![],
            Line::Point(coords) => vec![coords],
            Line::Horizontal(start, end) => compute_range(start.x, end.x)
                .iter()
                .map(|x| Coords { x: *x, y: start.y })
                .collect_vec(),
            Line::Vertical(start, end) => compute_range(start.y, end.y)
                .iter()
                .map(|y| Coords { x: start.x, y: *y })
                .collect_vec(),
            Line::Diagonal(start, end) => compute_range(start.x, end.x)
                .iter()
                .zip(compute_range(start.y, end.y).iter())
                .map(|(x, y)| Coords { x: *x, y: *y })
                .collect_vec(),
        };

        Self { points }
    }
}

pub fn is_horizontal(start: &Coords, end: &Coords) -> bool {
    start.x != end.x && start.y == end.y
}

pub fn is_vertical(start: &Coords, end: &Coords) -> bool {
    start.x == end.x && start.y != end.y
}

pub fn is_diagonal(start: &Coords, end: &Coords) -> bool {
    start.x != end.x && start.y != end.y && ((start.x - end.x).abs() != (start.y - end.y).abs())
}

impl From<(Coords, Coords)> for Line {
    fn from((start, end): (Coords, Coords)) -> Self {
        match (start, end) {
            (start, end) if is_vertical(&start, &end) => Self::Vertical(start, end),
            (start, end) if is_horizontal(&start, &end) => Self::Horizontal(start, end),
            (start, end) if is_diagonal(&start, &end) => Self::Diagonal(start, end),
            _ => Self::Invalid,
        }
    }
}
