use std::ops::Sub;
use crate::Vector2;

impl Sub for Vector2
{
    type Output = Vector2;

    fn sub(self, b: Vector2) -> Self::Output
    {
        return Vector2::new(self.x() - b.x(), self.y() - b.y());
    }
}