use crate::EPSILON;
use crate::Vector3;
use crate::Point;

mod operators
{
    pub mod div;
    pub mod mul;
    pub mod add;
    pub mod sub;
    pub mod neg;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuple
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple
{
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self
    {
        return Tuple
        {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64
    {
        return self.x;
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64
    {
        return self.y;
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64
    {
        return self.z;
    }

    #[allow(dead_code)]
    pub fn w(&self) -> f64
    {
        return self.w;
    }

    pub fn set_x(&mut self, x: f64)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64)
    {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f64)
    {
        self.z = z;
    }

    pub fn set_w(&mut self, w: f64)
    {
        self.w = w;
    }

    pub fn to_vector(&self) -> Vector3
    {
        return Vector3::new(self.x, self.y, self.z);
    }

    pub fn to_point(&self) -> Point
    {
        return Point::new(self.x, self.y, self.z);
    }

    #[allow(dead_code)]
    pub fn equal_approx(&self, other: Tuple) -> bool
    {
        let x_equal = (self.x - other.x).abs() < EPSILON;
        let y_equal = (self.y - other.y).abs() < EPSILON;
        let z_equal = (self.z - other.z).abs() < EPSILON;
        let w_equal = (self.w - other.w).abs() < EPSILON;
        return x_equal && y_equal && z_equal && w_equal;
    }

    #[allow(dead_code)]
    pub fn from(p: Tuple) -> Self
    {
        return Tuple
        {
            x: p.x(),
            y: p.y(),
            z: p.z(),
            w: p.w()
        }
    }
}