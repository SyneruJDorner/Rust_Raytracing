#![allow(clippy::flat_map_identity)]
#![allow(clippy::needless_range_loop)]

use crate::EPSILON;

mod operators
{
    pub mod div;
    pub mod mul;
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix
{
    pub matrix: [[f64; 4]; 4]
}

impl Matrix
{
    #[allow(dead_code)]
    pub fn new() -> Matrix
    {
        Matrix
        {
            matrix: [[1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[allow(dead_code)]
    pub fn set(m: [[f64; 4]; 4]) -> Matrix
    {
        Matrix
        {
            matrix: m
        }
    }

    #[allow(dead_code)]
    pub fn zero() -> Matrix
    {
        Matrix
        {
            matrix: [[0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0]]
        }
    }

    #[allow(dead_code)]
    pub fn identity() -> Matrix
    {
        Matrix
        {
            matrix: [[1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[allow(dead_code)]
    pub fn transpose(&self) -> Matrix
    {
        let mut result = Matrix::new();
        for i in 0..4
        {
            for j in 0..4
            {
                result.matrix[j][i] = self.matrix[i][j];
            }
        }
		return result;
    }

    pub fn inverse(&self) -> Option<Matrix>
    {
        let mut result = Matrix::identity();
        let mut self_copy = *self;

        for column in 0..4
        {
            if self_copy.matrix[column][column].abs() < EPSILON
            {
                let mut larger = column;

                for row in 0..4
                {
                    if self_copy.matrix[row][column].abs() > self_copy.matrix[larger][column].abs()
                    {
                        larger = row;
                    }
                }

                if larger == column
                {
                    return None;
                }

                self_copy.matrix.swap(column, larger);
                result.matrix.swap(column, larger);
            }

            for row in 0..4
            {
                if row == column
                {
                    continue;
                }

                let coeff = self_copy.matrix[row][column] / self_copy.matrix[column][column];

                if coeff != 0.0
                {
                    for j in 0..4
                    {
                        self_copy.matrix[row][j] -= coeff * self_copy.matrix[column][j];
                        result.matrix[row][j] -= coeff * result.matrix[column][j];
                    }

                    self_copy.matrix[row][column] = 0.0;
                }
            }
        }

        for row in 0..4
        {
            for column in 0..4
            {
                result.matrix[row][column] /= self_copy.matrix[row][row];
            }
        }

        return Some(result);
    }

    #[allow(dead_code)]
    pub fn print_matrix(m: Matrix)
    {
        println!("{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n",
                    m.matrix[0][0], m.matrix[0][1], m.matrix[0][2], m.matrix[0][3],
                    m.matrix[1][0], m.matrix[1][1], m.matrix[1][2], m.matrix[1][3],
                    m.matrix[2][0], m.matrix[2][1], m.matrix[2][2], m.matrix[2][3],
                    m.matrix[3][0], m.matrix[3][1], m.matrix[3][2], m.matrix[3][3]);
    }

    pub fn equal_approx(&self, other: Self) -> bool
    {
        const EPSILON: f64 = 0.0001;
        self.matrix.iter().flat_map(|x| x).zip(other.matrix.iter().flat_map(|y| y)).all(|(x, y)| (x - y).abs() < EPSILON)
    }
}
