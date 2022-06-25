use std::ops::Sub;
use crate::Color;

impl Sub for Color
{
    type Output = Color;

    fn sub(self, b: Color) -> Self::Output
    {
        return Color::new(self.r() - b.r(), self.g() - b.g(), self.b() - b.b());
    }
}