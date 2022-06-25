use std::ops::Div;
use crate::Tuple;

impl Div<f64> for Tuple
{
    type Output = Tuple;

    fn div(self, b: f64) -> Self::Output
    {
        return Tuple
        {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
            w: self.w / b
        };
    }
}