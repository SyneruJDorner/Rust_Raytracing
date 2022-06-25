use std::ops;
use crate::Matrix;

impl ops::Div<f64> for Matrix
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Self::Output
    {
        let mut mat = Self::new();
        for y in 0..self.matrix.len()
        {
            for x in 0..self.matrix[y].len()
            {
                mat.matrix[y][x] = mat.matrix[y][x] / rhs;
            }
        }
        mat
    }
}