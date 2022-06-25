use std::ops::Neg;
use crate::Tuple;

impl Neg for Tuple
{
    type Output = Tuple;

    fn neg(self) -> Self::Output
    {
        return Tuple
        {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        };
    }
}