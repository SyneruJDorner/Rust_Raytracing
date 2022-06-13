use crate::settings::Settings;
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
    pub aspect_ratio: f32,
    pub fov: f32,
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
    pub fn set_camera_from_settings(&mut self) -> Camera
    {
        self.fov = Settings::get_fov() as f32;
        self.aspect_ratio = Settings::get_aspect_ratio();
        self.prepare_ray();
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(&mut self, vfov: f32) -> Camera
    {
        self.fov = vfov;
        self.prepare_ray();
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        self.prepare_ray();
        return *self;
    }
    
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vec3) -> Camera
    {
        self.transform.set_position(position);
        self.prepare_ray();
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: Vec3) -> Camera
    {
        self.transform.set_rotation(rotation);
        self.prepare_ray();
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Vec3) -> Camera
    {
        self.transform.set_scale(scale);
        self.prepare_ray();
        return *self;
    }

    //Calculates the ray relative to the cameras position and rotation
    pub fn trace(&mut self, world: HittableList)
    {
        println!("P3\n{} {}\n255", Settings::get_image_width(), Settings::get_image_height());

        for y in 0..Settings::get_image_height()
        {
            for x in 0..Settings::get_image_width()
            {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..Settings::get_samples_per_pixel()
                {
                    let angle: f32 = (deg2rad((self.fov * 0.5).into()) as f64).tan() as f32;
                    let x = (2.0 * (x as f32 + 0.5) / Settings::get_image_width() as f32 - 1.0) * self.aspect_ratio * angle;
                    let y = (1.0 - 2.0 * (y as f32 + 0.5) / Settings::get_image_height() as f32) * angle;
                    let dir: Vec3 = self.ray.dir_matrix.multiply_dir_matrix(Vec3::new(x, y, -1.0)).normalize();
                    self.ray.direction = dir;
                    self.ray.origin = self.transform.position;
                    let backgorund_colour = Vec3::new(0.0, 0.0, 0.0);
                    let new_color = pixel_color + Ray::calcaulte_ray(&self.ray, &backgorund_colour, &world, Settings::get_max_depth());
                    pixel_color = new_color;
                }
                write_color(pixel_color, Settings::get_samples_per_pixel());
            }
        }
    }
}