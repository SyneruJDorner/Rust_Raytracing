use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::degrees_to_radians;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera
{
    transform: Transform,
    //position: Vec3,
    look_at: std::option::Option<Vec3>,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f32,
    fov: f32,
    world_up: Vec3
}

impl Camera
{
    #[allow(dead_code)]
    pub fn new() -> Camera
    {
        let start_aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = start_aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let start_transform = Transform::new();
        let start_look_at: std::option::Option<Vec3> = None;
        let start_horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let start_vertical = Vec3::new(0.0, viewport_height, 0.0);
        let start_lower_left_corner = start_transform.position - start_horizontal / 2.0 - start_vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        let start_fov = degrees_to_radians(90.0);
        let start_world_up = Vec3::new(0.0, 1.0, 0.0);

        let camera = Camera 
        {
            transform: start_transform,
            look_at: start_look_at,
            horizontal: start_horizontal,
            vertical: start_vertical,
            lower_left_corner: start_lower_left_corner,
            aspect_ratio:  start_aspect_ratio,
            fov: start_fov,
            world_up: start_world_up
        };
        return camera.update_camera();
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(mut self, vfov: f32) -> Camera
    {
        self.fov = vfov;
        return self.update_camera();
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        return self.update_camera();
    }

    #[allow(dead_code)]
    pub fn set_position(mut self, position: Vec3) -> Camera
    {
        self.transform.position = position;
        return self.update_camera();
    }

    #[allow(dead_code)]
    pub fn set_rotation(mut self, rotation: Vec3) -> Camera
    {
        self.transform.rotation = rotation;
        self.look_at = Some(rotation);
        return self.update_camera();
    }

    #[allow(dead_code)]
    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        return Ray::new(self.transform.position, self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.transform.position);
    }

    #[allow(dead_code)]
    fn update_camera(mut self) -> Camera
    {
        let theta = self.fov.to_radians();
        //let h = (theta / 2.0).tan();
        let half_height  = (theta / 2.0).tan();
        let half_width  = self.aspect_ratio * half_height;
        let start_position = self.transform.position;

        if self.look_at.is_some()
        {
            let w = (self.transform.position - self.look_at.unwrap()).normalize();
            let u = (Vec3::cross(&self.world_up, &w)).normalize();
            let v = Vec3::cross(&w, &u);

            let start_lower_left_corner = start_position - (u * half_width) - (v * half_height) - w;
            let start_horizontal = u * 2.0 * half_width;
            let start_vertical = v * 2.0 * half_height;
            let world_up = Vec3::new(0.0, 1.0, 0.0);

            self.transform.position = start_position;
            self.horizontal = start_horizontal;
            self.vertical = start_vertical;
            self.lower_left_corner = start_lower_left_corner;
            self.world_up = world_up;
            
            return self;
        }

        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let start_position = Vec3::new(0.0, 0.0, 0.0);
        let start_horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let start_vertical = Vec3::new(0.0, viewport_height, 0.0);
        let start_lower_left_corner = start_position - start_horizontal / 2.0 - start_vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        self.transform.position = start_position;
        self.horizontal = start_horizontal;
        self.vertical = start_vertical;
        self.lower_left_corner = start_lower_left_corner;

        return self;
    }
}