use std::fmt;
use uuid::Uuid;

use crate::Settings;
use crate::Matrix;
use crate::{Vector2, Point};
use crate::Vector3;
use crate::Color;
use crate::Ray;
use crate::random_float;
use map_3d::deg2rad;
use crate::Camera;
use pbr::ProgressBar;
use crate::HittableList;
//use crate::PostProcessing;

const HEADER_SIZE: usize = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Draw
{
	pixels: Vec<[f64; 3]>
}

pub enum RenderingTechnique
{
	Default,
	Checkerboard,
}

impl fmt::Display for RenderingTechnique
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
        match *self
		{
            RenderingTechnique::Default => write!(f, "Default"),
            RenderingTechnique::Checkerboard => write!(f, "Checkerboard"),
        }
    }
}

impl Draw
{	
    #[allow(dead_code)]
    pub fn new() -> Draw
	{
		let pixels_capacity: usize = Settings::get_image_width() as usize * Settings::get_image_height() as usize;
		let mut pixels = Vec::with_capacity(pixels_capacity);
		pixels.resize(pixels_capacity, [0.0, 0.0, 0.0]);
		Draw { pixels: pixels }
    }

	#[allow(dead_code)]
	pub fn get_pixels(&self) -> &Vec<[f64; 3]>
	{
		return &self.pixels;
	}

	// Convert the f64 RGB values in 'pixels' to Strings in 'final_pixels'
	pub fn get_pixels_as_str(&mut self) -> Vec<String>
	{
		let image_width = Settings::get_image_width() as usize;
        let image_height = Settings::get_image_height() as usize;
		let pixel_array_size = HEADER_SIZE + image_width * image_height;
		let mut final_pixels = Vec::with_capacity(pixel_array_size);
		
        final_pixels.push("P3".to_string());
        final_pixels.push(format!("{} {}", image_width, image_height));
        final_pixels.push("255".to_string());
        final_pixels.resize(pixel_array_size, String::new());

		for (index, pixel) in self.pixels.iter().enumerate()
		{
			let r = (pixel[0] * 255.0).round() as i32;
			let g = (pixel[1] * 255.0).round() as i32;
			let b = (pixel[2] * 255.0).round() as i32;
			final_pixels[HEADER_SIZE + index] = format!("{} {} {}", r, g, b);
		}
		return final_pixels;
	}

	#[allow(dead_code)]
	pub fn pixel(&mut self, x: u32, y: u32, pixel_color: Color, samples: u32)
	{
		if x < Settings::get_image_width() && y < Settings::get_image_height()
		{
			let pixel_index = (y * Settings::get_image_width() + x) as usize;
			
			// Further check to ensure that pixel_index is within bounds (for extra safety)
			if pixel_index < self.pixels.len()
			{
				self.pixels[pixel_index] = Color::store_color(pixel_color, samples);
			}
		}
		
	}

	#[allow(dead_code)]
	pub fn line(&mut self, mut from: Vector2, mut to: Vector2, color: Color)
	{
		let mut m: f64 = 0.0;

		if from.x() != to.x()
		{
			m = (to.y() - from.y()) / (to.x() - from.x());
		}

		if from.x() != to.x() && m.abs() <= 1.0
		{
			//Horizantal line calcation (y = mx + b)
			if from.x() > to.x()
			{
				std::mem::swap(&mut from, &mut to);
			}

			let b = from.y() - (m * from.x());

			for x in (from.x() as u32)..(to.x() as u32)
			{
				let y = (m * x as f64) + b;
				self.pixel(x, y as u32, color, 1);
			}
		}
		else
		{
			//Vertical line calcaultions (x = my + b)
			if from.y() > to.y()
			{
				std::mem::swap(&mut from, &mut to);
			}

			let m = (to.x() - from.x()) / (to.y() - from.y());
			let b = from.x() - (m * from.y());

			for y in (from.y() as u32)..(to.y() as u32)
			{
				let x = (m * y as f64) + b;
				self.pixel(x as u32, y, color, 1);
			}
		}
	}

	#[allow(dead_code)]
	pub fn draw_frame(&mut self)
	{
		let image_width = Settings::get_image_width() as f64;
		let image_height = Settings::get_image_height() as f64;

		self.line(Vector2::new(0.0 * image_width, 	0.0 * image_height),  	Vector2::new(0.125 * image_width, 0.167 * image_height),	Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.0 * image_width, 	1.0 * image_height),   	Vector2::new(0.125 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(1.0 * image_width, 	0.0 * image_height),    Vector2::new(0.875 * image_width, 0.167 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(1.0 * image_width, 	1.0 * image_height),   	Vector2::new(0.875 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		
		self.line(Vector2::new(0.125 * image_width, 0.167 * image_height),  Vector2::new(0.875 * image_width, 0.167 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.125 * image_width, 0.167 * image_height),  Vector2::new(0.125 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.875 * image_width, 0.167 * image_height),  Vector2::new(0.875 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.125 * image_width, 0.834 * image_height),  Vector2::new(0.875 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
	}

	#[allow(dead_code)]
	pub fn draw_crosshair(&mut self)
	{
		let image_width = Settings::get_image_width() as f64;
		let image_height = Settings::get_image_height() as f64;
		let center = Vector2::new(image_width / 2.0, image_height / 2.0);
		let aspect_ratio = Settings::get_aspect_ratio();

		//Draw crosshair include aspact ratio
		self.line(Vector2::new(center.x() - (0.03 * aspect_ratio * image_width), center.y()), Vector2::new(center.x() - 0.015 * aspect_ratio * image_width, center.y()), Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(center.x() + (0.03 * aspect_ratio * image_width), center.y()), Vector2::new(center.x() + (0.015 * aspect_ratio * image_width), center.y()), Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(center.x(), center.y() - (0.03 * aspect_ratio * image_height)), Vector2::new(center.x(), center.y() - 0.015 * aspect_ratio * image_height), Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(center.x(), center.y() + (0.03 * aspect_ratio * image_height)), Vector2::new(center.x(), center.y() + 0.015 * aspect_ratio * image_height), Color::new(0.0, 1.0, 0.0));
	}

	#[allow(dead_code)]
	pub fn position_to_pixel(&self, point: Point, projection_matrix: Matrix) -> Vector2
	{
		let mut point_vec = Point::new(point.x(), point.y(), point.z());
		point_vec = projection_matrix * point_vec;
		let point_vec = point_vec / point_vec.w();
		return Vector2::new(point_vec.x(), point_vec.y())
	}

	pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color>
	{
		let width = Settings::get_image_width();
		if x >= width || y >= self.pixels.len() as u32 / width {
			return None;
		}
		let pixel_index = (y * width + x) as usize;

		// Get the RGB values from the self.pixels vector
		let rgb: [f64; 3] = self.pixels[pixel_index];
	
		// Create and return a new Color using the RGB values
		return Some(Color::new(rgb[0], rgb[1], rgb[2]))
	}
	
	pub fn prepare_ray(&mut self, camera: &mut Camera, x_in: f64, y_in: f64, inv_width: f64, inv_height: f64, angle: f64) -> Ray
	{
		let u = 2.0 * (x_in + random_float(0.0, 1.0)) * inv_width;
		let v = 2.0 * (y_in + random_float(0.0, 1.0)) * inv_height;
	
		let x = (u - 1.0) * camera.aspect_ratio * angle;
		let y = (1.0 - v) * angle;
	
		let inverse = camera.transform.transform.inverse().unwrap();
		let direction = inverse * (Vector3::new(x, y, -1.0)).normalize();
		let origin = camera.transform.position;
		Ray::new(origin, direction, Uuid::nil())
	}

	#[allow(dead_code)]
	pub fn render(&mut self, camera: &mut Camera, world: &HittableList)
	{
		if Settings::get_rendering_method().to_lowercase() == RenderingTechnique::Default.to_string().to_lowercase()
		{
			self.render_default(camera, &world);
		}
		else if Settings::get_rendering_method().to_lowercase() == RenderingTechnique::Checkerboard.to_string().to_lowercase()
		{
			self.render_checkerboard(camera, &world);
		}
	}

	#[allow(dead_code)]
	pub fn render_default(&mut self, camera: &mut Camera, world: &HittableList)
	{
		let mut pb: ProgressBar<std::io::Stdout> = ProgressBar::new(100);
		let image_width = Settings::get_image_width() as f64;
		let image_height = Settings::get_image_height() as f64;
		let samples_per_pixel = Settings::get_samples_per_pixel();
		let inv_width = 1.0 / image_width;
		let inv_height = 1.0 / image_height;
		let angle = (deg2rad((camera.fov * 0.5).into()) as f64).tan();

		// Rendering Phase
		for y in 0..image_height as u32
		{
			for x in 0..image_width as u32
			{
				let mut pixel_color = Color::new(0.0, 0.0, 0.0);
				for _ in 0..samples_per_pixel
				{
					let ray = self.prepare_ray(camera, x as f64, y as f64, inv_width, inv_height, angle);
					pixel_color += Ray::calculate_ray(&ray, &world, Settings::get_max_depth());
				}

				self.pixel(x, y, pixel_color, samples_per_pixel);
			}
			pb.set((y as f64 / image_height * 100.0).ceil() as u64);
		}
		pb.finish();
	}

	#[allow(dead_code)]
	pub fn render_checkerboard(&mut self, camera: &mut Camera, world: &HittableList)
	{
		let mut pb: ProgressBar<std::io::Stdout> = ProgressBar::new(100);

		let image_width = Settings::get_image_width();
		let image_height = Settings::get_image_height();
		let inv_width = 1.0 / image_width as f64;
		let inv_height = 1.0 / image_height as f64;
		let angle = (deg2rad((camera.fov * 0.5).into()) as f64).tan();

		// First pass: Render checkerboard
		for y in 0..image_height
		{
			for x in 0..image_width
			{
				// Render pixels if they are part of the checkerboard pattern
				if (x + y) % 2 == 0
				{
					let mut pixel_color = Color::new(0.0, 0.0, 0.0);
					for _ in 0..Settings::get_samples_per_pixel()
					{
						//Write message to console
						let ray = self.prepare_ray(camera, x as f64, y as f64, inv_width, inv_height, angle);
						pixel_color += Ray::calculate_ray(&ray, &world, Settings::get_max_depth());
					}

					self.pixel(x, y, pixel_color, Settings::get_samples_per_pixel());
				}
			}
			pb.set((y as f64 / image_height as f64 * 100.0).ceil() as u64);
		}

		// Second pass: Fill in the pixels based on neighbors
		for y in 0..image_height
		{
			for x in 0..image_width
			{
				// If this pixel was not part of the checkerboard pattern
				if (x + y) % 2 != 0 {
					let neighbors = [
						(x.saturating_sub(1), y), // Left
						(x.saturating_add(1), y), // Right
						(x, y.saturating_sub(1)), // Top
						(x, y.saturating_add(1))  // Bottom
					];

					let mut average_color = Color::new(0.0, 0.0, 0.0);
					let mut valid_neighbors = 0;

					for &(nx, ny) in &neighbors
					{
						if let Some(color) = self.get_pixel(nx, ny)
						{
							// Use the color
							average_color += color;
							valid_neighbors += 1;
						}
					}

					if valid_neighbors > 0
					{
						self.pixel(x, y, average_color, valid_neighbors);
					}
				}
			}
			pb.set((y as f64 / image_height as f64 * 100.0).ceil() as u64);
		}

		pb.finish();
	}
}