use itertools::Itertools;

use crate::util::{Line, Path};

#[derive(Debug)]
pub struct Map {
    matrix: Vec<Vec<Vec<usize>>>,
    lines: Vec<Line>,
}

impl Map {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let lines = Vec::new();
        let mut matrix = Vec::new();

        for _ in 0..size_y {
            let mut row: Vec<Vec<usize>> = Vec::new();

            for _ in 0..size_x {
                let point: Vec<usize> = Vec::new();
                row.push(point);
            }

            matrix.push(row);
        }

        Self { lines, matrix }
    }

    pub fn find_intersection(&self, min_intersections: usize) -> Vec<((usize, usize), Vec<usize>)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, lines)| ((x, y), lines.clone()))
            })
            .filter(|(_, lines)| lines.len() >= min_intersections)
            .collect_vec()
    }

    pub fn draw_point(&mut self, x: i64, y: i64) {
        let line_index = self.lines.len();

        let point = self
            .matrix
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();

        point.push(line_index);
    }

    pub fn draw_line(&mut self, line: Line) {
        let path: Path = line.into();

        for point in path.points {
            self.draw_point(point.x, point.y);
        }
    }
}
