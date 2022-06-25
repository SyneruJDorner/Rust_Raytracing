use std::ops::Add;
use crate::Point;
use crate::Vector3;

impl Add for Point
{
    type Output = Point;

    fn add(self, b: Point) -> Self::Output
    {
        return Point::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
    }
}

impl Add<Vector3> for Point
{
    type Output = Point;

    fn add(self, b: Vector3) -> Self::Output
    {
        return Point::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
    }
}

impl Add<f64> for Point
{
    type Output = Point;

    fn add(self, b: f64) -> Self::Output
    {
        return Point::new(self.x() + b, self.y() + b, self.z() + b);
    }
}