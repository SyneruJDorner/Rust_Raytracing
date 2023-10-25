use std::ops::Mul;

use crate::Matrix;
use crate::Point;
use crate::Vector3;
use crate::AABB;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform
{
    pub transform: Matrix,
    pub position: Point,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub aabb_bounds: AABB,
}

impl Transform
{
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        return Transform
        {
            transform: Matrix::identity(),
            position: Point::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            aabb_bounds: AABB::new()
        };
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) -> Self
    {
        let mut transform_matrix = Matrix::identity();
        transform_matrix.matrix[0][3] = x;
        transform_matrix.matrix[1][3] = y;
        transform_matrix.matrix[2][3] = z;
        self.position = Point::new(x, y, z);
        self.transform = transform_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) -> Self
    {
        let mut scale_matrix = Matrix::identity();
        scale_matrix.matrix[0][0] = x;
        scale_matrix.matrix[1][1] = y;
        scale_matrix.matrix[2][2] = z;
        self.scale = Vector3::new(x, y, z);
        self.transform = scale_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, x: f64, y: f64, z: f64) -> Self
    {
        self.set_rotation_x(x);
        self.set_rotation_y(y);
        self.set_rotation_z(z);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation_x(&mut self, x: f64) -> Self
    {
        let angle_x = x.to_radians();

        //Handle X Rotation
        let mut rotate_x_matrix = Matrix::identity();
        rotate_x_matrix.matrix[1][1] = angle_x.cos();
        rotate_x_matrix.matrix[1][2] = -angle_x.sin();
        rotate_x_matrix.matrix[2][1] = angle_x.sin();
        rotate_x_matrix.matrix[2][2] = angle_x.cos();

        self.rotation.set_x(x);
        self.transform = rotate_x_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation_y(&mut self, y: f64) -> Self
    {
        let angle_y = y.to_radians();

        //Handle X Rotation
        let mut rotate_y_matrix = Matrix::identity();
        rotate_y_matrix.matrix[0][0] = angle_y.cos();
        rotate_y_matrix.matrix[0][2] = angle_y.sin();
        rotate_y_matrix.matrix[2][0] = -angle_y.sin();
        rotate_y_matrix.matrix[2][2] = angle_y.cos();

        self.rotation.set_y(y);
        self.transform = rotate_y_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation_z(&mut self, z: f64) -> Self
    {
        let angle_z = z.to_radians();

        //Handle X Rotation
        let mut rotate_z_matrix = Matrix::identity();
        rotate_z_matrix.matrix[0][0] = angle_z.cos();
        rotate_z_matrix.matrix[0][1] = -angle_z.sin();
        rotate_z_matrix.matrix[1][0] = angle_z.sin();
        rotate_z_matrix.matrix[1][1] = angle_z.cos();

        self.rotation.set_z(z);
        self.transform = rotate_z_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_shearing(&mut self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self
    {
        let mut shearing_matrix = Matrix::identity();
        shearing_matrix.matrix[0][1] = x_y;
        shearing_matrix.matrix[0][2] = x_z;
        shearing_matrix.matrix[1][0] = y_x;
        shearing_matrix.matrix[1][2] = y_z;
        shearing_matrix.matrix[2][0] = z_x;
        shearing_matrix.matrix[2][1] = z_y;
        self.transform = shearing_matrix * self.transform;
        self.aabb_bounds = AABB::calcalute_aabb_bounds(self);
        return *self;
    }

    pub fn rotation_matrix(&self) -> Matrix
    {
        let angle_x = self.rotation.x().to_radians();
        let mut rotate_x_matrix = Matrix::identity();
        rotate_x_matrix.matrix[1][1] = angle_x.cos();
        rotate_x_matrix.matrix[1][2] = -angle_x.sin();
        rotate_x_matrix.matrix[2][1] = angle_x.sin();
        rotate_x_matrix.matrix[2][2] = angle_x.cos();

        let angle_y = self.rotation.y().to_radians();
        let mut rotate_y_matrix = Matrix::identity();
        rotate_y_matrix.matrix[0][0] = angle_y.cos();
        rotate_y_matrix.matrix[0][2] = angle_y.sin();
        rotate_y_matrix.matrix[2][0] = -angle_y.sin();
        rotate_y_matrix.matrix[2][2] = angle_y.cos();

        let angle_z = self.rotation.z().to_radians();
        let mut rotate_z_matrix = Matrix::identity();
        rotate_z_matrix.matrix[0][0] = angle_z.cos();
        rotate_z_matrix.matrix[0][1] = -angle_z.sin();
        rotate_z_matrix.matrix[1][0] = angle_z.sin();
        rotate_z_matrix.matrix[1][1] = angle_z.cos();

        // Combining all rotation matrices to represent the current rotation state.
        rotate_x_matrix * rotate_y_matrix * rotate_z_matrix
    }

    
    pub fn normal_transform(&self) -> Option<Matrix>
    {
        // Assuming your Matrix struct has an inverse() method.
        let inverse = self.transform.inverse();
        
        match inverse {
            Some(inverse_matrix) => { return Some(inverse_matrix.transpose()) }, // If the inverse exists, compute and return its transpose.
            None => { return None } // If the inverse does not exist, return None.
        }
    }

    pub fn transform_normal(&self, normal: &Vector3) -> Option<Vector3>
    {
        let nt = self.normal_transform();
        match nt
        {
            Some(matrix) => { return Some(matrix * (*normal)) },
            None => { return None }
        }
    }
}

impl Default for Transform
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl From<Transform> for Matrix
{
    fn from(t: Transform) -> Self
    {
        return t.transform;
    }
}

impl Mul<Point> for Transform
{
    type Output = Point;

    fn mul(self, b: Point) -> Self::Output
    {
        return self.transform * b;
    }
}

impl Mul<Vector3> for Transform
{
    type Output = Vector3;

    fn mul(self, b: Vector3) -> Self::Output
    {
        return self.transform * b;
    }
}