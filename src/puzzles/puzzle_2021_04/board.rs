use crate::util::matrix_transpose;

type CheckMatrix = Vec<Vec<(i64, bool)>>;

#[derive(Debug)]
pub struct Board {
    pub matrix: CheckMatrix,
    has_won: bool,
}

impl Board {
    pub fn new(input: &[&str]) -> Self {
        let matrix = input
            .iter()
            .map(|col| {
                col.split_whitespace()
                    .map(|n| (n.parse::<i64>().unwrap(), false))
                    .collect()
            })
            .collect();

        Self {
            matrix,
            has_won: false,
        }
    }

    pub fn mark_number(&mut self, numb: i64) {
        let entry = self.matrix.iter_mut().flatten().find(|(n, _)| *n == numb);

        if let Some((_, is_checked)) = entry {
            *is_checked = true;

            if Self::check_rows(&self.matrix) {
                self.has_won = true;
                return;
            }

            if Self::check_cols(&self.matrix) {
                self.has_won = true;
            }
        };
    }

    pub fn has_won(&self) -> bool {
        self.has_won
    }

    pub fn unmarked_numbers(&self) -> impl Iterator<Item = i64> + '_ {
        self.matrix
            .iter()
            .flatten()
            .filter_map(|(n, is_checked)| if *is_checked { None } else { Some(*n) })
    }

    fn check_rows(matrix: &[Vec<(i64, bool)>]) -> bool {
        matrix
            .iter()
            .map(|row| row.iter().all(|(_, is_checked)| *is_checked))
            .any(|is_complete| is_complete)
    }

    fn check_cols(matrix: &[Vec<(i64, bool)>]) -> bool {
        let transposed = matrix_transpose(matrix);

        Self::check_rows(&transposed)
    }
}
