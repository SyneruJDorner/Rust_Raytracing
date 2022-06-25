use std::ops::Div;
use crate::Vector3;

impl Div<f64> for Vector3
{
    type Output = Vector3;

    fn div(self, b: f64) -> Self::Output
    {
        return Vector3::new(self.x() / b, self.y() / b, self.z() / b);
    }
}

impl Div<Vector3> for Vector3
{
    type Output = Vector3;

    fn div(self, b: Vector3) -> Self::Output
    {
        return Vector3::new(self.x() / b.x(), self.y() / b.x(), self.z() / b.x());
    }
}