use std::ops::Neg;
use crate::Vector3;

impl Neg for Vector3
{
    type Output = Vector3;

    fn neg(self) -> Self::Output
    {
        return Vector3::new(-self.x(), -self.y(), -self.z());
    }
}