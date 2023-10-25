use std::ops;
use crate::Matrix;

impl ops::Div<f64> for Matrix
{
    type Output = Matrix;
    fn div(self, b: f64) -> Self::Output
    {
        let mut mat = Self::new();
        for y in 0..self.matrix.len()
        {
            for x in 0..self.matrix[y].len()
            {
                mat.matrix[y][x] = self.matrix[y][x] / b;
            }
        }
        return mat;
    }
}

impl ops::Div<Matrix> for f64
{
    type Output = Matrix;
    fn div(self, b: Matrix) -> Matrix
    {
        let mut mat = Matrix::new();
        for y in 0..b.matrix.len()
        {
            for x in 0..b.matrix[y].len()
            {
                mat.matrix[y][x] = (1.0 / self) * b.matrix[y][x];
            }
        }

        return mat;
    }
}
