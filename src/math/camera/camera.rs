use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera
{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera
{
    pub fn new() -> Camera
    {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let start_origin = Vec3::new(0.0, 0.0, 0.0);
        let start_horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let start_vertical = Vec3::new(0.0, viewport_height, 0.0);
        let start_lower_left_corner = start_origin - start_horizontal / 2.0 - start_vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera 
        {
            origin: start_origin,
            horizontal: start_horizontal,
            vertical: start_vertical,
            lower_left_corner: start_lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin);
    }
}