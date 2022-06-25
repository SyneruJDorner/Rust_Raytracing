use std::ops::Mul;
use crate::Tuple;

impl Mul for Tuple
{
    type Output = Tuple;

    fn mul(self, b: Tuple) -> Self::Output
    {
        return Tuple
        {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
            w: self.w * b.w
        };
    }
}

impl Mul<Tuple> for f64
{
    type Output = Tuple;

    fn mul(self, b: Tuple) -> Self::Output
    {
        return Tuple
        {
            x: self * b.x,
            y: self * b.y,
            z: self * b.z,
            w: self * b.w
        };
    }
}

impl Mul<f64> for Tuple
{
    type Output = Tuple;

    fn mul(self, b: f64) -> Self::Output
    {
        return Tuple
        {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
            w: self.w * b
        };
    }
}