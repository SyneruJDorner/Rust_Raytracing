use std::ops;
use crate::matrix4x4::Matrix4x4;
use crate::vec3::Vec3;

//#region Operator *
impl ops::Mul<Matrix4x4> for Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();
        // Cache the invariants in registers
        let mut x = self.matrix[0][0];
        let mut y = self.matrix[0][1];
        let mut z = self.matrix[0][2];
        let mut w = self.matrix[0][3];
        // Perform the operation on the first row
        result.matrix[0][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[0][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[0][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[0][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        // Repeat for all the b.matrix rows
        x = self.matrix[1][0];
        y = self.matrix[1][1];
        z = self.matrix[1][2];
        w = self.matrix[1][3];
        result.matrix[1][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[1][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[1][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[1][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[2][0];
        y = self.matrix[2][1];
        z = self.matrix[2][2];
        w = self.matrix[2][3];
        result.matrix[2][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[2][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[2][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[2][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[3][0];
        y = self.matrix[3][1];
        z = self.matrix[3][2];
        w = self.matrix[3][3];
        result.matrix[3][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[3][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[3][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[3][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        return result;
    }
}

impl ops::Mul<&Matrix4x4> for Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();
        // Cache the invariants in registers
        let mut x = self.matrix[0][0];
        let mut y = self.matrix[0][1];
        let mut z = self.matrix[0][2];
        let mut w = self.matrix[0][3];
        // Perform the operation on the first row
        result.matrix[0][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[0][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[0][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[0][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        // Repeat for all the b.matrix rows
        x = self.matrix[1][0];
        y = self.matrix[1][1];
        z = self.matrix[1][2];
        w = self.matrix[1][3];
        result.matrix[1][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[1][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[1][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[1][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[2][0];
        y = self.matrix[2][1];
        z = self.matrix[2][2];
        w = self.matrix[2][3];
        result.matrix[2][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[2][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[2][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[2][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[3][0];
        y = self.matrix[3][1];
        z = self.matrix[3][2];
        w = self.matrix[3][3];
        result.matrix[3][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[3][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[3][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[3][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        return result;
    }
}

impl ops::Mul<Matrix4x4> for &Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();
        // Cache the invariants in registers
        let mut x = self.matrix[0][0];
        let mut y = self.matrix[0][1];
        let mut z = self.matrix[0][2];
        let mut w = self.matrix[0][3];
        // Perform the operation on the first row
        result.matrix[0][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[0][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[0][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[0][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        // Repeat for all the b.matrix rows
        x = self.matrix[1][0];
        y = self.matrix[1][1];
        z = self.matrix[1][2];
        w = self.matrix[1][3];
        result.matrix[1][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[1][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[1][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[1][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[2][0];
        y = self.matrix[2][1];
        z = self.matrix[2][2];
        w = self.matrix[2][3];
        result.matrix[2][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[2][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[2][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[2][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[3][0];
        y = self.matrix[3][1];
        z = self.matrix[3][2];
        w = self.matrix[3][3];
        result.matrix[3][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[3][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[3][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[3][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        return result;
    }
}

impl ops::Mul<&Matrix4x4> for &Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let mut result: Matrix4x4 = Matrix4x4::identity();
        // Cache the invariants in registers
        let mut x = self.matrix[0][0];
        let mut y = self.matrix[0][1];
        let mut z = self.matrix[0][2];
        let mut w = self.matrix[0][3];
        // Perform the operation on the first row
        result.matrix[0][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[0][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[0][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[0][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        // Repeat for all the b.matrix rows
        x = self.matrix[1][0];
        y = self.matrix[1][1];
        z = self.matrix[1][2];
        w = self.matrix[1][3];
        result.matrix[1][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[1][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[1][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[1][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[2][0];
        y = self.matrix[2][1];
        z = self.matrix[2][2];
        w = self.matrix[2][3];
        result.matrix[2][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[2][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[2][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[2][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        x = self.matrix[3][0];
        y = self.matrix[3][1];
        z = self.matrix[3][2];
        w = self.matrix[3][3];
        result.matrix[3][0] = (b.matrix[0][0] * x) + (b.matrix[1][0] * y) + (b.matrix[2][0] * z) + (b.matrix[3][0] * w);
        result.matrix[3][1] = (b.matrix[0][1] * x) + (b.matrix[1][1] * y) + (b.matrix[2][1] * z) + (b.matrix[3][1] * w);
        result.matrix[3][2] = (b.matrix[0][2] * x) + (b.matrix[1][2] * y) + (b.matrix[2][2] * z) + (b.matrix[3][2] * w);
        result.matrix[3][3] = (b.matrix[0][3] * x) + (b.matrix[1][3] * y) + (b.matrix[2][3] * z) + (b.matrix[3][3] * w);
        return result;
    }
}

impl ops::Mul<f32> for Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: f32) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<f32> for &Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: f32) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<&f32> for Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: &f32) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<&f32> for &Matrix4x4
{
    type Output = Matrix4x4;
    fn mul(self, b: &f32) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * b;
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix4x4> for f32
{
    type Output = Matrix4x4;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix4x4> for &f32
{
    type Output = Matrix4x4;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix4x4> for f32
{
    type Output = Matrix4x4;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<&Matrix4x4> for &f32
{
    type Output = Matrix4x4;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self * b.matrix[i][j];
            }
        }
        return result;
    }
}

impl ops::Mul<Matrix4x4> for Vec3
{
    type Output = Vec3;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let x = b.matrix[0][0] * self.x + b.matrix[0][1] * self.y + b.matrix[0][2] * self.z + b.matrix[0][3];
        let y = b.matrix[1][0] * self.x + b.matrix[1][1] * self.y + b.matrix[1][2] * self.z + b.matrix[1][3];
        let z = b.matrix[2][0] * self.x + b.matrix[2][1] * self.y + b.matrix[2][2] * self.z + b.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<Matrix4x4> for &Vec3
{
    type Output = Vec3;
    fn mul(self, b: Matrix4x4) -> Self::Output
    {
        let x = b.matrix[0][0] * self.x + b.matrix[0][1] * self.y + b.matrix[0][2] * self.z + b.matrix[0][3];
        let y = b.matrix[1][0] * self.x + b.matrix[1][1] * self.y + b.matrix[1][2] * self.z + b.matrix[1][3];
        let z = b.matrix[2][0] * self.x + b.matrix[2][1] * self.y + b.matrix[2][2] * self.z + b.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<&Matrix4x4> for Vec3
{
    type Output = Vec3;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let x = b.matrix[0][0] * self.x + b.matrix[0][1] * self.y + b.matrix[0][2] * self.z + b.matrix[0][3];
        let y = b.matrix[1][0] * self.x + b.matrix[1][1] * self.y + b.matrix[1][2] * self.z + b.matrix[1][3];
        let z = b.matrix[2][0] * self.x + b.matrix[2][1] * self.y + b.matrix[2][2] * self.z + b.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<&Matrix4x4> for &Vec3
{
    type Output = Vec3;
    fn mul(self, b: &Matrix4x4) -> Self::Output
    {
        let x = b.matrix[0][0] * self.x + b.matrix[0][1] * self.y + b.matrix[0][2] * self.z + b.matrix[0][3];
        let y = b.matrix[1][0] * self.x + b.matrix[1][1] * self.y + b.matrix[1][2] * self.z + b.matrix[1][3];
        let z = b.matrix[2][0] * self.x + b.matrix[2][1] * self.y + b.matrix[2][2] * self.z + b.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<Vec3> for Matrix4x4
{
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Self::Output
    {
        let x = self.matrix[0][0] * b.x + self.matrix[0][1] * b.y + self.matrix[0][2] * b.z + self.matrix[0][3];
        let y = self.matrix[1][0] * b.x + self.matrix[1][1] * b.y + self.matrix[1][2] * b.z + self.matrix[1][3];
        let z = self.matrix[2][0] * b.x + self.matrix[2][1] * b.y + self.matrix[2][2] * b.z + self.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<Vec3> for &Matrix4x4
{
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Self::Output
    {
        let x = self.matrix[0][0] * b.x + self.matrix[0][1] * b.y + self.matrix[0][2] * b.z + self.matrix[0][3];
        let y = self.matrix[1][0] * b.x + self.matrix[1][1] * b.y + self.matrix[1][2] * b.z + self.matrix[1][3];
        let z = self.matrix[2][0] * b.x + self.matrix[2][1] * b.y + self.matrix[2][2] * b.z + self.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<&Vec3> for Matrix4x4
{
    type Output = Vec3;
    fn mul(self, b: &Vec3) -> Self::Output
    {
        let x = self.matrix[0][0] * b.x + self.matrix[0][1] * b.y + self.matrix[0][2] * b.z + self.matrix[0][3];
        let y = self.matrix[1][0] * b.x + self.matrix[1][1] * b.y + self.matrix[1][2] * b.z + self.matrix[1][3];
        let z = self.matrix[2][0] * b.x + self.matrix[2][1] * b.y + self.matrix[2][2] * b.z + self.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}

impl ops::Mul<&Vec3> for &Matrix4x4
{
    type Output = Vec3;
    fn mul(self, b: &Vec3) -> Self::Output
    {
        let x = self.matrix[0][0] * b.x + self.matrix[0][1] * b.y + self.matrix[0][2] * b.z + self.matrix[0][3];
        let y = self.matrix[1][0] * b.x + self.matrix[1][1] * b.y + self.matrix[1][2] * b.z + self.matrix[1][3];
        let z = self.matrix[2][0] * b.x + self.matrix[2][1] * b.y + self.matrix[2][2] * b.z + self.matrix[2][3];
        return Vec3::new(x, y, z);
    }
}
//#endregion