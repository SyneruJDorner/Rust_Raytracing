use std::ops::Mul;
use crate::Color;

impl Mul for Color
{
    type Output = Color;

    fn mul(self, b: Color) -> Self::Output
    {
        return Color::new(self.r() * b.r(), self.g() * b.g(), self.b() * b.b());
    }
}

impl Mul<f64> for Color
{
    type Output = Color;

    fn mul(self, b: f64) -> Self::Output
    {
        return Color::new(self.r() * b, self.g() * b, self.b() * b);
    }
}

impl Mul<Color> for f64
{
    type Output = Color;

    fn mul(self, b: Color) -> Self::Output
    {
        return Color::new(b.r() * self, b.g() * self, b.b() * self);
    }
}
