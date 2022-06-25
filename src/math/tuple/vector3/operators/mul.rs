use std::ops::Mul;
use crate::Vector3;
use crate::Point;

impl Mul<Vector3> for f64
{
    type Output = Vector3;

    fn mul(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self * b.x(), self * b.y(), self * b.z());
    }
}

impl Mul<f64> for Vector3
{
    type Output = Vector3;

    fn mul(self, b: f64) -> Self::Output
    {
        return Vector3::new(self.x() * b, self.y() * b, self.z() * b);
    }
}

impl Mul<Vector3> for Vector3
{
    type Output = Vector3;

    fn mul(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
    }
}

impl Mul<Vector3> for Point
{
    type Output = Vector3;

    fn mul(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self.x() * b.x(), self.y() * b.y(), self.z() * b.z());
    }
}