use libm::atan2f;

use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix4x4
{
    pub matrix: [[f32; 4]; 4]
}

impl Matrix4x4
{
    #[allow(dead_code)]
    pub fn new() -> Matrix4x4
    {
        Matrix4x4
        {
            matrix: [[1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[allow(dead_code)]
    pub fn zero() -> Matrix4x4
    {
        Matrix4x4
        {
            matrix: [[0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0]]
        }
    }

    #[allow(dead_code)]
    pub fn identity() -> Matrix4x4
    {
        Matrix4x4
        {
            matrix: [[1.0, 0.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0, 0.0],
                     [0.0, 0.0, 1.0, 0.0],
                     [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[allow(dead_code)]
    pub fn translate(mut self, translation: Vec3) -> Matrix4x4
	{
		self.matrix[0][3] = translation.x;
		self.matrix[1][3] = translation.y;
		self.matrix[2][3] = translation.z;
		self.matrix[0][0] = 1.0;
		self.matrix[1][1] = 1.0;
		self.matrix[2][2] = 1.0;
		self.matrix[3][3] = 1.0;
		return self;
	}

    #[allow(dead_code)]
    pub fn scale(mut self, scale: Vec3) -> Matrix4x4
	{
		self.matrix[0][0] = scale.x;
		self.matrix[1][1] = scale.y;
		self.matrix[2][2] = scale.z;
		self.matrix[3][3] = 1.0;
		return self;
	}

    #[allow(dead_code)]
    pub fn rotate_x(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.cos();
        let s = angle.sin();
        self.matrix[1][1] = c;
        self.matrix[1][2] = -s;
        self.matrix[2][1] = s;
        self.matrix[2][2] = c;
        return self;
    }

    #[allow(dead_code)]
    pub fn rotate_y(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.cos();
        let s = angle.sin();
        self.matrix[0][0] = c;
        self.matrix[0][2] = s;
        self.matrix[2][0] = -s;
        self.matrix[2][2] = c;
        return self;
    }

    #[allow(dead_code)]
    pub fn rotate_z(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.cos();
        let s = angle.sin();
        self.matrix[0][0] = c;
        self.matrix[0][1] = -s;
        self.matrix[1][0] = s;
        self.matrix[1][1] = c;
        return self;
    }

    #[allow(dead_code)]
    pub fn rotate(r: f32, i: f32, j: f32, k: f32) -> Matrix4x4
	{
		let mut m = Matrix4x4::new();
        m.matrix[0][0] = 1.0 - 2.0 * j * j - 2.0 * k * k;
        m.matrix[0][1] = 2.0 * i * j - 2.0 * k * r;
        m.matrix[0][2] = 2.0 * i * k + 2.0 * j * r;
        m.matrix[0][3] = 0.0;
        
        m.matrix[1][0] = 2.0 * i * j + 2.0 * r * k;
        m.matrix[1][1] = 1.0 - 2.0 * i * i - 2.0 * k * k;
        m.matrix[1][2] = 2.0 * j * k - 2.0 * r * i;
        m.matrix[1][3] = 0.0;

        m.matrix[2][0] = 2.0 * i * k - 2.0 * r * j;
        m.matrix[2][1] = 2.0 * j * k + 2.0 * r * i;
        m.matrix[2][2] = 1.0 - 2.0 * i * i - 2.0 * j * j;
        m.matrix[2][3] = 0.0;

        m.matrix[3][0] = 0.0;
        m.matrix[3][1] = 0.0;
        m.matrix[3][2] = 0.0;
        m.matrix[3][3] = 1.0;

		return m;
	}

    #[allow(dead_code)]
    pub fn determinant(m: &Matrix4x4) -> f32
	{
        return  m.matrix[0][3] * m.matrix[1][2] * m.matrix[2][1] * m.matrix[3][0] -
                m.matrix[0][2] * m.matrix[1][3] * m.matrix[2][1] * m.matrix[3][0] -
                m.matrix[0][3] * m.matrix[1][1] * m.matrix[2][2] * m.matrix[3][0] +
                m.matrix[0][1] * m.matrix[1][3] * m.matrix[2][2] * m.matrix[3][0] +
                m.matrix[0][2] * m.matrix[1][1] * m.matrix[2][3] * m.matrix[3][0] -
                m.matrix[0][1] * m.matrix[1][2] * m.matrix[2][3] * m.matrix[3][0] -
                m.matrix[0][3] * m.matrix[1][2] * m.matrix[2][0] * m.matrix[3][1] +
                m.matrix[0][2] * m.matrix[1][3] * m.matrix[2][0] * m.matrix[3][1] +
                m.matrix[0][3] * m.matrix[1][0] * m.matrix[2][2] * m.matrix[3][1] -
                m.matrix[0][0] * m.matrix[1][3] * m.matrix[2][2] * m.matrix[3][1] -
                m.matrix[0][2] * m.matrix[1][0] * m.matrix[2][3] * m.matrix[3][1] +
                m.matrix[0][0] * m.matrix[1][2] * m.matrix[2][3] * m.matrix[3][1] +
                m.matrix[0][3] * m.matrix[1][1] * m.matrix[2][0] * m.matrix[3][2] -
                m.matrix[0][1] * m.matrix[1][3] * m.matrix[2][0] * m.matrix[3][2] -
                m.matrix[0][3] * m.matrix[1][0] * m.matrix[2][1] * m.matrix[3][2] +
                m.matrix[0][0] * m.matrix[1][3] * m.matrix[2][1] * m.matrix[3][2] +
                m.matrix[0][1] * m.matrix[1][0] * m.matrix[2][3] * m.matrix[3][2] -
                m.matrix[0][0] * m.matrix[1][1] * m.matrix[2][3] * m.matrix[3][2] -
                m.matrix[0][2] * m.matrix[1][1] * m.matrix[2][0] * m.matrix[3][3] +
                m.matrix[0][1] * m.matrix[1][2] * m.matrix[2][0] * m.matrix[3][3] +
                m.matrix[0][2] * m.matrix[1][0] * m.matrix[2][1] * m.matrix[3][3] -
                m.matrix[0][0] * m.matrix[1][2] * m.matrix[2][1] * m.matrix[3][3] -
                m.matrix[0][1] * m.matrix[1][0] * m.matrix[2][2] * m.matrix[3][3] +
                m.matrix[0][0] * m.matrix[1][1] * m.matrix[2][2] * m.matrix[3][3];
    }

    #[allow(dead_code)]
    pub fn inverse(mut self) -> Matrix4x4
    {
        let mut inverted_matrix = Matrix4x4::new();
        inverted_matrix.matrix[0][0] = self.matrix[1][2] * self.matrix[2][3] * self.matrix[3][1] - self.matrix[1][3] * self.matrix[2][2] * self.matrix[3][1] + self.matrix[1][3] * self.matrix[2][1] * self.matrix[3][2] - self.matrix[1][1] * self.matrix[2][3] * self.matrix[3][2] - self.matrix[1][2] * self.matrix[2][1] * self.matrix[3][3] + self.matrix[1][1] * self.matrix[2][2] * self.matrix[3][3];
        inverted_matrix.matrix[0][1] = self.matrix[0][3] * self.matrix[2][2] * self.matrix[3][1] - self.matrix[0][2] * self.matrix[2][3] * self.matrix[3][1] - self.matrix[0][3] * self.matrix[2][1] * self.matrix[3][2] + self.matrix[0][1] * self.matrix[2][3] * self.matrix[3][2] + self.matrix[0][2] * self.matrix[2][1] * self.matrix[3][3] - self.matrix[0][1] * self.matrix[2][2] * self.matrix[3][3];
        inverted_matrix.matrix[0][2] = self.matrix[0][2] * self.matrix[1][3] * self.matrix[3][1] - self.matrix[0][3] * self.matrix[1][2] * self.matrix[3][1] + self.matrix[0][3] * self.matrix[1][1] * self.matrix[3][2] - self.matrix[0][1] * self.matrix[1][3] * self.matrix[3][2] - self.matrix[0][2] * self.matrix[1][1] * self.matrix[3][3] + self.matrix[0][1] * self.matrix[1][2] * self.matrix[3][3];
        inverted_matrix.matrix[0][3] = self.matrix[0][3] * self.matrix[1][2] * self.matrix[2][1] - self.matrix[0][2] * self.matrix[1][3] * self.matrix[2][1] - self.matrix[0][3] * self.matrix[1][1] * self.matrix[2][2] + self.matrix[0][1] * self.matrix[1][3] * self.matrix[2][2] + self.matrix[0][2] * self.matrix[1][1] * self.matrix[2][3] - self.matrix[0][1] * self.matrix[1][2] * self.matrix[2][3];
        inverted_matrix.matrix[1][0] = self.matrix[1][3] * self.matrix[2][2] * self.matrix[3][0] - self.matrix[1][2] * self.matrix[2][3] * self.matrix[3][0] - self.matrix[1][3] * self.matrix[2][0] * self.matrix[3][2] + self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][2] + self.matrix[1][2] * self.matrix[2][0] * self.matrix[3][3] - self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][3];
        inverted_matrix.matrix[1][1] = self.matrix[0][2] * self.matrix[2][3] * self.matrix[3][0] - self.matrix[0][3] * self.matrix[2][2] * self.matrix[3][0] + self.matrix[0][3] * self.matrix[2][0] * self.matrix[3][2] - self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][2] - self.matrix[0][2] * self.matrix[2][0] * self.matrix[3][3] + self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][3];
        inverted_matrix.matrix[1][2] = self.matrix[0][3] * self.matrix[1][2] * self.matrix[3][0] - self.matrix[0][2] * self.matrix[1][3] * self.matrix[3][0] - self.matrix[0][3] * self.matrix[1][0] * self.matrix[3][2] + self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][2] + self.matrix[0][2] * self.matrix[1][0] * self.matrix[3][3] - self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][3];
        inverted_matrix.matrix[1][3] = self.matrix[0][2] * self.matrix[1][3] * self.matrix[2][0] - self.matrix[0][3] * self.matrix[1][2] * self.matrix[2][0] + self.matrix[0][3] * self.matrix[1][0] * self.matrix[2][2] - self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][2] - self.matrix[0][2] * self.matrix[1][0] * self.matrix[2][3] + self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][3];
        inverted_matrix.matrix[2][0] = self.matrix[1][1] * self.matrix[2][3] * self.matrix[3][0] - self.matrix[1][3] * self.matrix[2][1] * self.matrix[3][0] + self.matrix[1][3] * self.matrix[2][0] * self.matrix[3][1] - self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][1] - self.matrix[1][1] * self.matrix[2][0] * self.matrix[3][3] + self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][3];
        inverted_matrix.matrix[2][1] = self.matrix[0][3] * self.matrix[2][1] * self.matrix[3][0] - self.matrix[0][1] * self.matrix[2][3] * self.matrix[3][0] - self.matrix[0][3] * self.matrix[2][0] * self.matrix[3][1] + self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][1] + self.matrix[0][1] * self.matrix[2][0] * self.matrix[3][3] - self.matrix[0][0] * self.matrix[2][1] * self.matrix[3][3];
        inverted_matrix.matrix[2][2] = self.matrix[0][1] * self.matrix[1][3] * self.matrix[3][0] - self.matrix[0][3] * self.matrix[1][1] * self.matrix[3][0] + self.matrix[0][3] * self.matrix[1][0] * self.matrix[3][1] - self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][1] - self.matrix[0][1] * self.matrix[1][0] * self.matrix[3][3] + self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][3];
        inverted_matrix.matrix[2][3] = self.matrix[0][3] * self.matrix[1][1] * self.matrix[2][0] - self.matrix[0][1] * self.matrix[1][3] * self.matrix[2][0] - self.matrix[0][3] * self.matrix[1][0] * self.matrix[2][1] + self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][1] + self.matrix[0][1] * self.matrix[1][0] * self.matrix[2][3] - self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][3];
        inverted_matrix.matrix[3][0] = self.matrix[1][2] * self.matrix[2][1] * self.matrix[3][0] - self.matrix[1][1] * self.matrix[2][2] * self.matrix[3][0] + self.matrix[1][1] * self.matrix[2][0] * self.matrix[3][2] - self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][2] - self.matrix[1][2] * self.matrix[2][0] * self.matrix[3][3] + self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][3];
        inverted_matrix.matrix[3][1] = self.matrix[0][2] * self.matrix[2][2] * self.matrix[3][0] - self.matrix[0][2] * self.matrix[2][0] * self.matrix[3][2] - self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][2] + self.matrix[0][0] * self.matrix[2][0] * self.matrix[3][2] + self.matrix[0][2] * self.matrix[2][0] * self.matrix[3][3] - self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][3];
        inverted_matrix.matrix[3][2] = self.matrix[0][1] * self.matrix[1][2] * self.matrix[3][0] - self.matrix[0][2] * self.matrix[1][1] * self.matrix[3][0] + self.matrix[0][2] * self.matrix[1][0] * self.matrix[3][1] - self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][1] - self.matrix[0][1] * self.matrix[1][0] * self.matrix[3][2] + self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][2];
        inverted_matrix.matrix[3][3] = self.matrix[0][2] * self.matrix[1][1] * self.matrix[2][0] - self.matrix[0][1] * self.matrix[1][2] * self.matrix[2][0] - self.matrix[0][2] * self.matrix[1][0] * self.matrix[2][1] + self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][1] + self.matrix[0][1] * self.matrix[1][0] * self.matrix[2][2] - self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][2];
        self = inverted_matrix;
        return self;
    }

    #[allow(dead_code)]
    pub fn  mul_float(self, scale: f32) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[i][j] * scale;
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn mul_vec3(self, v: Vec3) -> Vec3
    {
        let x = self.matrix[0][0] * v.x + self.matrix[0][1] * v.y + self.matrix[0][2] * v.z + self.matrix[0][3];
        let y = self.matrix[1][0] * v.x + self.matrix[1][1] * v.y + self.matrix[1][2] * v.z + self.matrix[1][3];
        let z = self.matrix[2][0] * v.x + self.matrix[2][1] * v.y + self.matrix[2][2] * v.z + self.matrix[2][3];
        return Vec3::new(x, y, z);
    }

    #[allow(dead_code)]
    pub fn to_vec3(self) -> Vec3
	{
        let x = self.matrix[0][0] + self.matrix[0][1] + self.matrix[0][2] + self.matrix[0][3];
        let y = self.matrix[1][0] + self.matrix[1][1] + self.matrix[1][2] + self.matrix[1][3];
        let z = self.matrix[2][0] + self.matrix[2][1] + self.matrix[2][2] + self.matrix[2][3];
        return Vec3::new(x, y, z);
	}

    #[allow(dead_code)]
    pub fn ortho_normalize(mut self) -> Matrix4x4
	{
		let right = Vec3::new(self.matrix[0][0], self.matrix[1][0], self.matrix[2][0]).normalize();
		self.matrix[0][0] = right.x;
		self.matrix[1][0] = right.y;
		self.matrix[2][0] = right.z;

        let up = Vec3::new(self.matrix[0][1], self.matrix[1][1], self.matrix[2][1]).normalize();
		self.matrix[0][1] = up.x;
		self.matrix[1][1] = up.y;
		self.matrix[2][1] = up.z;

        let dir = Vec3::new(self.matrix[0][2], self.matrix[1][2], self.matrix[2][2]).normalize();
		self.matrix[0][2] = dir.x;
		self.matrix[1][2] = dir.y;
		self.matrix[2][2] = dir.z;

        return self;
	}

    #[allow(dead_code)]
    pub fn extract_translation(self) -> Vec3
    {
        let x = self.matrix[0][3];
        let y = self.matrix[1][3];
        let z = self.matrix[2][3];
        return Vec3::new(x, y, z);
    }

    #[allow(dead_code)]
    pub fn extract_scale(self) -> Vec3
	{
        let x = Vec3::new(self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]).length();
        let y = Vec3::new(self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]).length();
        let z = Vec3::new(self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]).length();
        return Vec3::new(x, y, z);
	}

    //Add in the future
    // pub fn extract_rotation(self) -> Quaternion {
    //     let q: Quaternion = Quaternion::new();
    //     let trace = self.matrix[0][0] + self.matrix[1][1] + self.matrix[2][2];

    //     if trace > 0.0
    //     {
    //         let s = 0.5 / sqrt(trace + 1.0);
	// 		q.r = 0.25 / s;
	// 		q.i = (self.matrix[2][1] - self.matrix[1][2]) * s;
	// 		q.j = (self.matrix[0][2] - self.matrix[2][0]) * s;
	// 		q.k = (self.matrix[1][0] - self.matrix[0][1]) * s;
    //     }
    //     else
    //     {
    //         if (self.matrix[0][0] > self.matrix[1][1] && self.matrix[0][0] > self.matrix[2][2])
	// 		{
	// 			let s = 2.0 * sqrtf(1.0 + self.matrix[0][0] - self.matrix[1][1] - self.matrix[2][2]);
	// 			q.r = (self.matrix[2][1] - self.matrix[1][2]) / s;
	// 			q.i = 0.25 * s;
	// 			q.j = (self.matrix[0][1] + self.matrix[1][0]) / s;
	// 			q.k = (self.matrix[0][2] + self.matrix[2][0]) / s;
	// 		}
	// 		else if (self.matrix[1][1] > self.matrix[2][2])
	// 		{
	// 			let s = 2.0 * sqrtf(1.0 + self.matrix[1][1] - self.matrix[0][0] - self.matrix[2][2]);
	// 			q.r = (self.matrix[0][2] - self.matrix[2][0]) / s;
	// 			q.i = (self.matrix[0][1] + self.matrix[1][0]) / s;
	// 			q.j = 0.25 * s;
	// 			q.k = (self.matrix[1][2] + self.matrix[2][1]) / s;
	// 		}
	// 		else
	// 		{
	// 			let s = 2.0 * sqrtf(1.0 + self.matrix[2][2] - self.matrix[0][0] - self.matrix[1][1]);
	// 			q.r = (self.matrix[1][0] - self.matrix[0][1]) / s;
	// 			q.i = (self.matrix[0][2] + self.matrix[2][0]) / s;
	// 			q.j = (self.matrix[1][2] + self.matrix[2][1]) / s;
	// 			q.k = 0.25 * s;
	// 		}
    //     }
    //     return q;
    // }

    // pub fn extract_rotation_x(self) -> f32
	// {
	// 	let rot: Quaternion = self.extract_rotation();
	// 	return rot.euler_angles().x;
	// }

    // pub fn extract_rotation_y(self) -> f32
	// {
	// 	let rot: Quaternion = self.extract_rotation();
	// 	return rot.euler_angles().y;
	// }

    // pub fn extract_rotation_z(self) -> f32
	// {
	// 	let rot: Quaternion = self.extract_rotation();
	// 	return rot.euler_angles().z;
	// }
    
    #[allow(dead_code)]
    pub fn decompose(self, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut Vec3)
    {
        let mut m = self.get_transpose();

        scale.x = Vec3::new(m.matrix[0][0], m.matrix[0][1], m.matrix[0][2]).length();
		scale.y = Vec3::new(m.matrix[1][0], m.matrix[1][1], m.matrix[1][2]).length();
		scale.z = Vec3::new(m.matrix[2][0], m.matrix[2][1], m.matrix[2][2]).length();

		m = m.ortho_normalize();

		rotation.x = atan2f(m.matrix[1][2], m.matrix[2][2]);
		rotation.y = atan2f(-m.matrix[0][2], (m.matrix[1][2] * m.matrix[1][2] + m.matrix[2][2] * m.matrix[2][2]).sqrt());
		rotation.z = atan2f(m.matrix[0][1], m.matrix[0][0]);

		translation.x = m.matrix[3][0];
		translation.y = m.matrix[3][1];
		translation.z = m.matrix[3][2];
    }

    #[allow(dead_code)]
    pub fn get_transpose(self) -> Matrix4x4
	{
		let mut result = Matrix4x4::new();
        for i in 0..4
        {
            for j in 0..4
            {
                result.matrix[i][j] = self.matrix[j][i];
            }
        }
		return result;
	}
}
