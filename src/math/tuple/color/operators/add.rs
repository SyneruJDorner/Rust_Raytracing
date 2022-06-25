use std::ops::{Add, AddAssign};
use crate::Color;

impl Add for Color
{
    type Output = Color;

    fn add(self, b: Color) -> Self
    {
        return Color::new(self.r() + b.r(), self.g() + b.g(), self.b() + b.b());
    }
}

impl AddAssign for Color
{
    fn add_assign(&mut self, b: Color)
    {
        *self = *self + b;
    }
}