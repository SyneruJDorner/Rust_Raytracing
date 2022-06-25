use std::ops::Add;
use crate::Tuple;

impl Add for Tuple
{
    type Output = Tuple;

    fn add(self, b: Tuple) -> Self::Output
    {
        return Tuple
        {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w
        };
    }
}