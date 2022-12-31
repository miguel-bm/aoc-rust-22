/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub struct Matrix2D<T: std::clone::Clone> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: std::clone::Clone> Matrix2D<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Matrix2D<T> {
        let data = vec![vec![default; cols]; rows];
        Matrix2D { rows, cols, data }
    }

    pub fn from_vec(data: Vec<Vec<T>>) -> Matrix2D<T> {
        let rows = data.len();
        let cols = data[0].len();

        // assert that all rows have the same length
        for row in data.iter() {
            assert_eq!(row.len(), cols);
        }

        Matrix2D { rows, cols, data }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row][col]
    }

    pub fn get_row(&self, row: usize) -> Vec<&T> {
        self.data[row].iter().collect()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
        self.data.iter().map(|row| row.iter().collect())
    }

    pub fn get_col(&self, col: usize) -> Vec<&T> {
        self.data.iter().map(|row| &row[col]).collect()
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.cols).map(move |col| self.get_col(col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    pub fn set_row(&mut self, row: usize, values: Vec<T>) {
        self.data[row] = values;
    }

    pub fn set_col(&mut self, col: usize, values: Vec<T>) {
        for (i, value) in values.iter().enumerate() {
            self.data[i][col] = value.clone();
        }
    }
    pub fn get_rotation(&self, turns: i32) -> Matrix2D<T> {
        let actual_turns = turns % 4;
        if actual_turns == 0 {
            return Matrix2D {
                rows: self.cols.clone(),
                cols: self.rows.clone(),
                data: self.data.clone(),
            };
        } else if actual_turns == 1 {
            let mut rotated = Matrix2D::new(self.cols, self.rows, self.data[0][0].clone());
            for i in 0..rotated.rows {
                for j in 0..rotated.cols {
                    rotated.set(i, j, self.get(self.rows - j - 1, i).clone());
                }
            }
            return rotated;
        } else {
            return self.get_rotation(actual_turns - 1).get_rotation(1);
        }
    }
}
