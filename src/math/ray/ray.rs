use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray
{
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray { origin: origin, direction: direction }
    }

    pub fn origin(self) -> Vec3
    {
        return self.origin;
    }

    pub fn direction(self) -> Vec3
    {
        return self.direction;
    }

    pub fn at(self, t: f32) -> Vec3
    {
        return self.origin + t * self.direction;
    }
}
