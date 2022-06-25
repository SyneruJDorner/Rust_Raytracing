use std::ops::Div;
use crate::Point;

impl Div<f64> for Point
{
    type Output = Point;

    fn div(self, b: f64) -> Self::Output
    {
        return Point::new(self.x() / b, self.y() / b, self.z() / b);
    }
}