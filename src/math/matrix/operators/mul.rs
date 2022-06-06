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

        // let a = self;
        // Matrix4x4 { 
        //     x: a.x * b.x,
        //     y: a.y * b.y,
        //     z: a.z * b.z
    }
}

// impl ops::Mul<&Matrix4x4> for Matrix4x4
// {
//     type Output = Matrix4x4;
//     fn mul(self, b: &Matrix4x4) -> Self::Output
//     {
//         let a = self;
//         Matrix4x4 { 
//             x: a.x * b.x,
//             y: a.y * b.y,
//             z: a.z * b.z
//         }
//     }
// }

// impl ops::Mul<Matrix4x4> for &Matrix4x4
// {
//     type Output = Matrix4x4;
//     fn mul(self, b: Matrix4x4) -> Self::Output
//     {
//         let a = self;
//         Matrix4x4 { 
//             x: a.x * b.x,
//             y: a.y * b.y,
//             z: a.z * b.z
//         }
//     }
// }

// impl ops::Mul<&Matrix4x4> for &Matrix4x4
// {
//     type Output = Matrix4x4;
//     fn mul(self, b: &Matrix4x4) -> Self::Output
//     {
//         let a = self;
//         Matrix4x4 { 
//             x: a.x * b.x,
//             y: a.y * b.y,
//             z: a.z * b.z
//         }
//     }
// }
/////////////////////////////////////////

/*
inline Matrix4x4 operator*(const Matrix4x4& m, const Matrix4x4& other)
{
	Matrix4x4 result;
	// Cache the invariants in registers
	float x = m(0, 0);
	float y = m(0, 1);
	float z = m(0, 2);
	float w = m(0, 3);
	// Perform the operation on the first row
	result.m[0][0] = (other(0, 0) * x) + (other(1, 0) * y) + (other(2, 0) * z) + (other(3, 0) * w);
	result.m[0][1] = (other(0, 1) * x) + (other(1, 1) * y) + (other(2, 1) * z) + (other(3, 1) * w);
	result.m[0][2] = (other(0, 2) * x) + (other(1, 2) * y) + (other(2, 2) * z) + (other(3, 2) * w);
	result.m[0][3] = (other(0, 3) * x) + (other(1, 3) * y) + (other(2, 3) * z) + (other(3, 3) * w);
	// Repeat for all the other rows
	x = m(1, 0);
	y = m(1, 1);
	z = m(1, 2);
	w = m(1, 3);
	result.m[1][0] = (other(0, 0) * x) + (other(1, 0) * y) + (other(2, 0) * z) + (other(3, 0) * w);
	result.m[1][1] = (other(0, 1) * x) + (other(1, 1) * y) + (other(2, 1) * z) + (other(3, 1) * w);
	result.m[1][2] = (other(0, 2) * x) + (other(1, 2) * y) + (other(2, 2) * z) + (other(3, 2) * w);
	result.m[1][3] = (other(0, 3) * x) + (other(1, 3) * y) + (other(2, 3) * z) + (other(3, 3) * w);
	x = m(2, 0);
	y = m(2, 1);
	z = m(2, 2);
	w = m(2, 3);
	result.m[2][0] = (other(0, 0) * x) + (other(1, 0) * y) + (other(2, 0) * z) + (other(3, 0) * w);
	result.m[2][1] = (other(0, 1) * x) + (other(1, 1) * y) + (other(2, 1) * z) + (other(3, 1) * w);
	result.m[2][2] = (other(0, 2) * x) + (other(1, 2) * y) + (other(2, 2) * z) + (other(3, 2) * w);
	result.m[2][3] = (other(0, 3) * x) + (other(1, 3) * y) + (other(2, 3) * z) + (other(3, 3) * w);
	x = m(3, 0);
	y = m(3, 1);
	z = m(3, 2);
	w = m(3, 3);
	result.m[3][0] = (other(0, 0) * x) + (other(1, 0) * y) + (other(2, 0) * z) + (other(3, 0) * w);
	result.m[3][1] = (other(0, 1) * x) + (other(1, 1) * y) + (other(2, 1) * z) + (other(3, 1) * w);
	result.m[3][2] = (other(0, 2) * x) + (other(1, 2) * y) + (other(2, 2) * z) + (other(3, 2) * w);
	result.m[3][3] = (other(0, 3) * x) + (other(1, 3) * y) + (other(2, 3) * z) + (other(3, 3) * w);
	return result;
}
*/