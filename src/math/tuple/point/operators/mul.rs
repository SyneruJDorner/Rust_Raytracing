use std::ops::Mul;
use crate::Point;
use crate::Vector3;

impl Mul<Point> for f64
{
    type Output = Point;

    fn mul(self, b: Point) -> Self::Output
    {
        return Point::new(self * b.x(), self * b.y(), self * b.z());
    }
}

impl Mul<Point> for Point
{
    type Output = Point;

    fn mul(self, b: Point) -> Self::Output
    {
        return Point::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
    }
}

impl Mul<Point> for Vector3
{
    type Output = Point;

    fn mul(self, b: Point) -> Self::Output
    {
        return Point::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
    }
}