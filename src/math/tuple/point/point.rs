use crate::Tuple;
use crate::Vector3;

mod operators
{
    pub mod div;
    pub mod mul;
    pub mod add;
    pub mod sub;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(Tuple);

impl Point
{
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Point
    {
        return Point(Tuple::new(x, y, z, 1.0));
    }

    #[allow(dead_code)]
    pub fn zero() -> Point
    {
        return Point(Tuple::new(0.0, 0.0, 0.0, 1.0));
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64
    {
        return self.0.x;
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64
    {
        return self.0.y;
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64
    {
        return self.0.z;
    }

    #[allow(dead_code)]
    pub fn w(&self) -> f64
    {
        return self.0.w;
    }

    #[allow(dead_code)]
    pub fn set_x(&mut self, x: f64)
    {
        self.0.x = x;
    }

    #[allow(dead_code)]
    pub fn set_y(&mut self, y: f64)
    {
        self.0.y = y;
    }

    #[allow(dead_code)]
    pub fn set_z(&mut self, z: f64)
    {
        self.0.z = z;
    }

    #[allow(dead_code)]
    pub fn set_w(&mut self, w: f64)
    {
        self.0.w = w;
    }

    #[allow(dead_code)]
    pub fn to_vector(&self) -> Vector3
    {
        return Vector3::new(self.x(), self.y(), self.z());
    }

    #[allow(dead_code)]
    pub fn to_tuple(&self) -> Tuple
    {
        return Tuple::new(self.x(), self.y(), self.z(), self.w());
    }

    #[allow(dead_code)]
    pub fn equal_approx(&self, other: Point) -> bool
    {
        return self.0.equal_approx(other.0);
    }
}

impl From<Point> for Tuple
{
    #[allow(dead_code)]
    fn from(p: Point) -> Self
    {
        return p.0;
    }
}