use std::ops::Add;
use crate::Point;
use crate::Vector3;

impl Add<Point> for Vector3
{
    type Output = Point;

    fn add(self, b: Point) -> Self::Output
    {
        return Point::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
    }
}

impl Add for Vector3
{
    type Output = Vector3;

    fn add(self, b: Vector3) -> Self
    {
        return Vector3::new(self.x() + b.x(), self.y() + b.y(), self.z() + b.z());
    }
}