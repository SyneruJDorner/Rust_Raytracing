use std::ops;
use crate::matrix4x4::Matrix4x4;

//#region Operator *
/////////////////////////////////////////
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