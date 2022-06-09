use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::hittablelist::HittableList;
use crate::color::write_color;
use crate::utils::degrees_to_radians;

use map_3d::deg2rad;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Camera
{
    pub transform: Transform,
    aspect_ratio: f32,
    pub fov: f32,
    world_up: Vec3,
    pub ray: Ray
}

impl Camera
{
    #[allow(dead_code)]
    pub fn new() -> Camera
    {
        Camera 
        {
            transform: Transform::new(),
            aspect_ratio:  degrees_to_radians(90.0),
            fov: degrees_to_radians(90.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            ray: Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
        }
    }

    pub fn prepare_ray(&mut self)
    {
        let ray_origin = self.transform.local_matrix.multiply_vec3_matrix(self.transform.position);
        self.ray = Ray::new(ray_origin, Vec3::new(0.0, 0.0, 0.0));
        self.ray.set_dir_matrix(self.transform.world_matrix * self.transform.local_matrix);
    }

    #[allow(dead_code)]
    pub fn print_camera(&self)
    {
        println!("Camera");
        println!("{} {} {}", self.transform.position.x, self.transform.position.y, self.transform.position.z);
        println!("{} {} {}", self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        println!("{} {} {}", self.transform.scale.x, self.transform.scale.y, self.transform.scale.z);
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(&mut self, vfov: f32) -> Camera
    {
        self.fov = vfov;
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        return *self;
    }
    
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vec3) -> Camera
    {
        self.transform.set_position(position);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: Vec3) -> Camera
    {
        self.transform.set_rotation(rotation);
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Vec3) -> Camera
    {
        self.transform.set_scale(scale);
        return *self;
    }

    //Calculates the ray relative to the cameras position and rotation
    pub fn trace(&mut self, world: HittableList, width: i32, height: i32, samples_per_pixel: i32, depth: i32)
    {
        println!("P3\n{} {}\n255", width, height);

        for y in 0..height
        {
            for x in 0..width
            {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel
                {
                    let angle: f32 = (deg2rad((self.fov * 0.5).into()) as f64).tan() as f32;
                    let x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * self.aspect_ratio * angle;
                    let y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * angle;
                    let dir: Vec3 = self.ray.dir_matrix.multiply_dir_matrix(Vec3::new(x, y, -1.0)).normalize();
                    self.ray.direction = dir;
                    let new_color = pixel_color + Ray::calcaulte_ray(&self.ray, &world, depth);
                    pixel_color = new_color;
                }
                write_color(pixel_color, samples_per_pixel);
            }
        }
    }
}