use std::ops::Div;
use crate::Color;

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, scalar: f64) -> Color {
        (Color::new(self.r() / scalar, self.g() / scalar, self.b() / scalar)).clamp()
    }
}