use crate::Settings;
use crate::{Vector2, Vector3};
use crate::Transform;
use crate::HittableList;
use crate::Draw;
use crate::clear_cmd;

use map_3d::deg2rad;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera
{
    pub height: u32,
    pub width: u32,
    pub pixel_size: f64,
    pub transform: Transform, //Veiw Matrix located in the transform struct
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

        let mut camera: Camera = Camera 
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
        draw.render(self, &world);

        //Apply any line debugging here!
        //draw.draw_frame();
        //draw.draw_crosshair();
        
        //Write all the array to a file
        let mut file = File::create("image.ppm").unwrap();
        for pixel in draw.get_pixels_as_str()
        {
            file.write_all(pixel.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        
        clear_cmd();
        println!("Image created!");
    }
}