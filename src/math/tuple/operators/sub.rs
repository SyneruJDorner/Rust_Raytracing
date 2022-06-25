use std::ops::Sub;
use crate::Tuple;

impl Sub for Tuple
{
    type Output = Tuple;

    fn sub(self, b: Tuple) -> Self::Output
    {
        return Tuple
        {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
            w: self.w - b.w
        };
    }
}