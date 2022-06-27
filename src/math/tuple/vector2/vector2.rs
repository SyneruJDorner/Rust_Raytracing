use crate::Tuple;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2(Tuple);

impl Vector2
{
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64) -> Vector2
    {
        return Vector2(Tuple::new(x, y, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn zero() -> Vector2
    {
        return Vector2(Tuple::new(0.0, 0.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn one() -> Vector2
    {
        return Vector2(Tuple::new(1.0, 1.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64
    {
        return self.0.x;
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64
    {
        return self.0.y;
    }

    #[allow(dead_code)]
    pub fn set_x(&mut self, x: f64)
    {
        self.0.x = x;
    }

    #[allow(dead_code)]
    pub fn set_y(&mut self, y: f64)
    {
        self.0.y = y;
    }
}

impl From<Vector2> for Tuple
{
    #[allow(dead_code)]
    fn from(v: Vector2) -> Self
    {
        return v.0;
    }
}