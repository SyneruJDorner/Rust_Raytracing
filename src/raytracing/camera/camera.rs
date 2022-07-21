use uuid::Uuid;

use crate::Settings;
use crate::random_float;
use crate::Vector3;
use crate::Color;
use crate::Ray;
use crate::Transform;
use crate::HittableList;
use crate::Draw;
use crate::clear_cmd;
use pbr::ProgressBar;

use map_3d::deg2rad;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera
{
    pub height: u32,
    pub width: u32,
    pub pixel_size: f64,
    pub transform: Transform,
    pub aspect_ratio: f64,
    pub fov: f64,
    pub half_width: f64,
    pub half_height: f64
}

impl Camera
{
    #[allow(dead_code)]
    pub fn new() -> Camera
    {
        let width = Settings::get_image_width();
        let height = Settings::get_image_height();
        let fov = 90.0;
        let angle: f64 = (deg2rad((fov * 0.5).into()) as f64).tan() as f64;
        let aspect_ratio = (width as f64) / (height as f64);
        let half_width = if aspect_ratio >= 1.0 { angle } else { angle * aspect_ratio };
        let half_height = if aspect_ratio >= 1.0 { angle / aspect_ratio } else { angle };
        let pixel_size = (half_width * 2.0) / height as f64;
        let transform = Transform::new();

        let mut camera = Camera 
        {
            height: height,
            width: width,
            pixel_size: pixel_size,
            transform: transform,
            aspect_ratio:  aspect_ratio,
            fov: fov,
            half_width: half_width,
            half_height: half_height
        };

        camera.set_camera_from_settings();

        return camera;
    }

    pub fn prepare_ray(&mut self, x_in: u32, y_in: u32) -> Ray
    {
        let u = 2.0 * (x_in as f64 + random_float(0.0, 1.0)) / (Settings::get_image_width() as f64);
        let v = 2.0 * (y_in as f64 + random_float(0.0, 1.0)) / (Settings::get_image_height() as f64);

        let angle: f64 = (deg2rad((self.fov * 0.5).into()) as f64).tan() as f64;
        let x = (u - 1.0) * self.aspect_ratio * angle;
        let y = (1.0 - v) * angle;

        let inverse = self.transform.transform.inverse().unwrap();
        let direction = inverse * (Vector3::new(x, y, -1.0)).normalize();
        let origin = self.transform.position;
        return Ray::new(origin, direction, Uuid::nil());
    }

    #[allow(dead_code)]
    pub fn set_camera_from_settings(&mut self) -> Camera
    {
        self.fov = Settings::get_fov() as f64;
        self.aspect_ratio = Settings::get_aspect_ratio();
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_field_of_view(&mut self, vfov: f64) -> Camera
    {
        self.fov = vfov;
        return *self;
    }

    #[allow(dead_code)]
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> Camera
    {
        self.aspect_ratio = aspect_ratio;
        return *self;
    }

    //Calculates the ray relative to the cameras position and rotation
    pub fn trace(&mut self, world: HittableList)
    {
        clear_cmd();
        let mut draw = Draw::new();
        let mut pb = ProgressBar::new(100);
        
        for y in 0..Settings::get_image_height()
        {
            for x in 0..Settings::get_image_width()
            {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..Settings::get_samples_per_pixel()
                {
                    let ray = self.prepare_ray(x, y);
                    pixel_color += Ray::calcaulte_ray(ray, &world, Settings::get_max_depth());
                }

                draw.pixel(x, y, pixel_color, Settings::get_samples_per_pixel());
            }
            pb.set((y as f64 / Settings::get_image_height() as f64 * 100.0).ceil() as u64);
        }
        pb.finish();

        //Apply any line debugging here!
        draw.draw_frame();

        //Write all the array to a file
        let mut file = File::create("image.ppm").unwrap();
        for pixel in draw.get_pixels()
        {
            file.write_all(pixel.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        
        clear_cmd();
        println!("Image created!");
    }
}