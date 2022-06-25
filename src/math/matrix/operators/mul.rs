use std::ops;
use crate::Matrix;
use crate::Tuple;
use crate::Vector3;
use crate::Point;

//#region Operator * (Matrix * Matrix)
impl ops::Mul<Matrix> for Matrix
{
    type Output = Matrix;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result: Matrix = Matrix::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] =   self.matrix[i][0] * b.matrix[0][j] +
                                        self.matrix[i][1] * b.matrix[1][j] +
                                        self.matrix[i][2] * b.matrix[2][j] +
                                        self.matrix[i][3] * b.matrix[3][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix> for Matrix
{
    type Output = Matrix;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result: Matrix = Matrix::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] =   self.matrix[i][0] * b.matrix[0][j] +
                                        self.matrix[i][1] * b.matrix[1][j] +
                                        self.matrix[i][2] * b.matrix[2][j] +
                                        self.matrix[i][3] * b.matrix[3][j];
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix> for &Matrix
{
    type Output = Matrix;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result: Matrix = Matrix::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] =   self.matrix[i][0] * b.matrix[0][j] +
                                        self.matrix[i][1] * b.matrix[1][j] +
                                        self.matrix[i][2] * b.matrix[2][j] +
                                        self.matrix[i][3] * b.matrix[3][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix> for &Matrix
{
    type Output = Matrix;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result: Matrix = Matrix::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] =   self.matrix[i][0] * b.matrix[0][j] +
                                        self.matrix[i][1] * b.matrix[1][j] +
                                        self.matrix[i][2] * b.matrix[2][j] +
                                        self.matrix[i][3] * b.matrix[3][j];
            }
        }
        return result;
    }
}
//#endregion

//#region Operator * (Matrix * f64)
impl ops::Mul<f64> for Matrix
{
    type Output = Matrix;
    fn mul(self, b: f64) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<f64> for &Matrix
{
    type Output = Matrix;
    fn mul(self, b: f64) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<&f64> for Matrix
{
    type Output = Matrix;
    fn mul(self, b: &f64) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<&f64> for &Matrix
{
    type Output = Matrix;
    fn mul(self, b: &f64) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix> for f64
{
    type Output = Matrix;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix> for &f64
{
    type Output = Matrix;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix> for f64
{
    type Output = Matrix;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix> for &f64
{
    type Output = Matrix;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}
//#endregion

//#region Operator * (Matrix * Tuple)
impl ops::Mul<Matrix> for Tuple
{
    type Output = Tuple;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Matrix> for &Tuple
{
    type Output = Tuple;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for Tuple
{
    type Output = Tuple;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for &Tuple
{
    type Output = Tuple;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Tuple> for Matrix
{
    type Output = Tuple;
    fn mul(self, b: Tuple) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<Tuple> for &Matrix
{
    type Output = Tuple;
    fn mul(self, b: Tuple) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Tuple> for Matrix
{
    type Output = Tuple;
    fn mul(self, b: &Tuple) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Tuple> for &Matrix
{
    type Output = Tuple;
    fn mul(self, b: &Tuple) -> Self::Output
    {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}
//#endregion

//#region Operator * (Matrix * Vector3)
impl ops::Mul<Matrix> for Vector3
{
    type Output = Vector3;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Matrix> for &Vector3
{
    type Output = Vector3;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for Vector3
{
    type Output = Vector3;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for &Vector3
{
    type Output = Vector3;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Vector3> for Matrix
{
    type Output = Vector3;
    fn mul(self, b: Vector3) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<Vector3> for &Matrix
{
    type Output = Vector3;
    fn mul(self, b: Vector3) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Vector3> for Matrix
{
    type Output = Vector3;
    fn mul(self, b: &Vector3) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Vector3> for &Matrix
{
    type Output = Vector3;
    fn mul(self, b: &Vector3) -> Self::Output
    {
        let mut result = Vector3::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}
//#endregion

//#region Operator * (Matrix * Point)
impl ops::Mul<Matrix> for Point
{
    type Output = Point;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Matrix> for &Point
{
    type Output = Point;
    fn mul(self, b: Matrix) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for Point
{
    type Output = Point;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<&Matrix> for &Point
{
    type Output = Point;
    fn mul(self, b: &Matrix) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(b.matrix[0][0] * self.x() + b.matrix[0][1] * self.y() + b.matrix[0][2] * self.z() + b.matrix[0][3] * self.w());
        result.set_y(b.matrix[1][0] * self.x() + b.matrix[1][1] * self.y() + b.matrix[1][2] * self.z() + b.matrix[1][3] * self.w());
        result.set_z(b.matrix[2][0] * self.x() + b.matrix[2][1] * self.y() + b.matrix[2][2] * self.z() + b.matrix[2][3] * self.w());
        result.set_w(b.matrix[3][0] * self.x() + b.matrix[3][1] * self.y() + b.matrix[3][2] * self.z() + b.matrix[3][3] * self.w());
        return result;
    }
}

impl ops::Mul<Point> for Matrix
{
    type Output = Point;
    fn mul(self, b: Point) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<Point> for &Matrix
{
    type Output = Point;
    fn mul(self, b: Point) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Point> for Matrix
{
    type Output = Point;
    fn mul(self, b: &Point) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}

impl ops::Mul<&Point> for &Matrix
{
    type Output = Point;
    fn mul(self, b: &Point) -> Self::Output
    {
        let mut result = Point::new(0.0, 0.0, 0.0);
        result.set_x(self.matrix[0][0] * b.x() + self.matrix[0][1] * b.y() + self.matrix[0][2] * b.z() + self.matrix[0][3] * b.w());
        result.set_y(self.matrix[1][0] * b.x() + self.matrix[1][1] * b.y() + self.matrix[1][2] * b.z() + self.matrix[1][3] * b.w());
        result.set_z(self.matrix[2][0] * b.x() + self.matrix[2][1] * b.y() + self.matrix[2][2] * b.z() + self.matrix[2][3] * b.w());
        result.set_w(self.matrix[3][0] * b.x() + self.matrix[3][1] * b.y() + self.matrix[3][2] * b.z() + self.matrix[3][3] * b.w());
        return result;
    }
}
//#endregion