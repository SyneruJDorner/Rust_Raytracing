use std::ops::Sub;
use crate::Point;
use crate::Vector3;

impl Sub for Point
{
    type Output = Vector3;

    fn sub(self, b: Point) -> Self::Output
    {
        return Vector3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
    }
}

impl Sub<Vector3> for Point
{
    type Output = Vector3;

    fn sub(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self.x() - b.x(), self.y() - b.y(), self.z() - b.z());
    }
}
