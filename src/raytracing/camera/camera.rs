use crate::vec3::Vec3;
//use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::degrees_to_radians;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera
{
    pub transform: Transform,
    aspect_ratio: f32,
    pub fov: f32,
    world_up: Vec3
}

impl Camera
{
    #[allow(dead_code)]
    pub fn new() -> Camera
    {
        let camera = Camera 
        {
            transform: Transform::new(),
            aspect_ratio:  degrees_to_radians(90.0),
            fov: degrees_to_radians(90.0),
            world_up: Vec3::new(0.0, 1.0, 0.0)
        };
        return camera;
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(mut self, vfov: f32) -> Camera
    {
        self.fov = vfov;
        return self;
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        return self;
    }
    
    #[allow(dead_code)]
    pub fn set_position(mut self, position: Vec3) -> Camera
    {
        self.transform.set_position(position);
        return self;
    }

    #[allow(dead_code)]
    pub fn set_rotation(mut self, rotation: Vec3) -> Camera
    {
        self.transform.set_rotation(rotation);
        return self;
    }

    #[allow(dead_code)]
    pub fn set_scale(mut self, scale: Vec3) -> Camera
    {
        self.transform.set_scale(scale);
        return self;
    }
}