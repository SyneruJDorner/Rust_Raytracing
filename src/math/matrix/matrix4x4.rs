use libm::atan2f;

use crate::vec3::Vec3;

mod operators {
    pub mod mul;
}

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
		self.matrix[3][0] = translation.x;
		self.matrix[3][1] = translation.y;
		self.matrix[3][2] = translation.z;
		// self.matrix[0][0] = 1.0;
		// self.matrix[1][1] = 1.0;
		// self.matrix[2][2] = 1.0;
		// self.matrix[3][3] = 1.0;
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

    //Equivilent to rolling
    //|1    0       0       x|
    //|0    cos()   -sin()  y|
    //|0    sin()   cos()   z|
    //|0    0       0       1|
    #[allow(dead_code)]
    pub fn rotate_x(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.to_radians().cos();
        let s = angle.to_radians().sin();
        self.matrix[1][1] = c;
        self.matrix[1][2] = -s;
        self.matrix[2][1] = s;
        self.matrix[2][2] = c;
        return self;
    }

    #[allow(dead_code)]
    pub fn rotate_y(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.to_radians().cos();
        let s = angle.to_radians().sin();
        self.matrix[0][0] = c;
        self.matrix[0][2] = s;
        self.matrix[2][0] = -s;
        self.matrix[2][2] = c;
        return self;
    }

    #[allow(dead_code)]
    pub fn rotate_z(mut self, angle: f32) -> Matrix4x4
    {
        let c = angle.to_radians().cos();
        let s = angle.to_radians().sin();
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

    #[allow(dead_code)]
    pub fn print_matrix(m: Matrix4x4)
    {
        println!("{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n{:.2}\t{:.2}\t{:.2}\t{:.2}\n",
                    m.matrix[0][0], m.matrix[0][1], m.matrix[0][2], m.matrix[0][3],
                    m.matrix[1][0], m.matrix[1][1], m.matrix[1][2], m.matrix[1][3],
                    m.matrix[2][0], m.matrix[2][1], m.matrix[2][2], m.matrix[2][3],
                    m.matrix[3][0], m.matrix[3][1], m.matrix[3][2], m.matrix[3][3]);
    }

    #[allow(dead_code)]
    pub fn multiply_dir_matrix(self, src: Vec3) -> Vec3
    {  
        let mut dst: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let a = src.x * self.matrix[0][0] + src.y * self.matrix[1][0] + src.z * self.matrix[2][0]; 
        let b = src.x * self.matrix[0][1] + src.y * self.matrix[1][1] + src.z * self.matrix[2][1]; 
        let c = src.x * self.matrix[0][2] + src.y * self.matrix[1][2] + src.z * self.matrix[2][2]; 
 
        dst.x = a; 
        dst.y = b; 
        dst.z = c; 

        return dst;
    }

    #[allow(dead_code)]
    pub fn multiply_vec3_matrix(self, src: Vec3) -> Vec3
    {
        let mut dst: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let a = src.x * self.matrix[0][0] + src.y * self.matrix[1][0] + src.z * self.matrix[2][0] + self.matrix[3][0]; 
        let b = src.x * self.matrix[0][1] + src.y * self.matrix[1][1] + src.z * self.matrix[2][1] + self.matrix[3][1];  
        let c = src.x * self.matrix[0][2] + src.y * self.matrix[1][2] + src.z * self.matrix[2][2] + self.matrix[3][2]; 
        let w = src.x * self.matrix[0][2] + src.y * self.matrix[1][2] + src.z * self.matrix[2][2] + self.matrix[3][3];

        dst.x = a / w; 
        dst.y = b / w; 
        dst.z = c / w; 

        return dst;
    }
}