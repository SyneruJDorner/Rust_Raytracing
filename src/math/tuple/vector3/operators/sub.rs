use std::ops::Sub;
use crate::Vector3;
use crate::Point;

impl Sub for Vector3
{
    type Output = Vector3;

    fn sub(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
    }
}

impl Sub<Point> for Vector3
{
    type Output = Vector3;

    fn sub(self, b: Point) -> Self::Output
    {
        return Vector3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
    }
}