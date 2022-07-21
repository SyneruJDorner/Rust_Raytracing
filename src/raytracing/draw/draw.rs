use crate::Settings;
use crate::Vector2;
use crate::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Draw
{
	pixels: Vec<String>
}

impl Draw
{
    #[allow(dead_code)]
    pub fn new() -> Draw
    {
		let header_size = 3;
        let pixel_array_size = header_size + Settings::get_image_width() * Settings::get_image_height();
        let mut new_pixels = vec![String::new(); pixel_array_size as usize];

		//Add header to the pixels array image
		new_pixels[0] = String::from("P3");
		new_pixels[1] = String::from(format!("{} {}", Settings::get_image_width(), Settings::get_image_height()));
		new_pixels[2] = String::from(format!("255"));

        let pixels = Draw
        {
			pixels: new_pixels
        };

        return pixels;
	}

	#[allow(dead_code)]
	pub fn get_pixels(&self) -> &Vec<String>
	{
		return &self.pixels;
	}

	#[allow(dead_code)]
	pub fn pixel(&mut self, x: u32, y: u32, pixel_color: Color, samples: u32)
	{
		let header_size = 3;
		let pixel_index = header_size + (y * Settings::get_image_width() + x);
		let color_string = Color::write_color(pixel_color, samples);

		if pixel_index >= (header_size + Settings::get_image_width() * Settings::get_image_height()) { return; }
		self.pixels[pixel_index as usize] = color_string;
	}

	#[allow(dead_code)]
	pub fn line(&mut self, mut from: Vector2, mut to: Vector2, color: Color)
	{
		let mut m = 0.0;

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
		
		self.line(Vector2::new(0.125 * image_width,	0.167 * image_height),  Vector2::new(0.875 * image_width, 0.167 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.125 * image_width, 0.167 * image_height),  Vector2::new(0.125 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.875 * image_width, 0.167 * image_height),  Vector2::new(0.875 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));
		self.line(Vector2::new(0.125 * image_width, 0.834 * image_height),  Vector2::new(0.875 * image_width, 0.834 * image_height),    Color::new(0.0, 1.0, 0.0));

        // draw.line(Vector2::new(0.0, 0.0),       Vector2::new(100.0, 100.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(0.0, 600.0),     Vector2::new(100.0, 500.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(800.0, 0.0),     Vector2::new(700.0, 100.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(800.0, 600.0),   Vector2::new(700.0, 500.0),     Color::new(0.0, 1.0, 0.0));
        
        // draw.line(Vector2::new(100.0, 100.0),   Vector2::new(700.0, 100.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(100.0, 100.0),   Vector2::new(100.0, 500.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(700.0, 100.0),   Vector2::new(700.0, 500.0),     Color::new(0.0, 1.0, 0.0));
        // draw.line(Vector2::new(100.0, 500.0),   Vector2::new(700.0, 500.0),     Color::new(0.0, 1.0, 0.0));
	}
}