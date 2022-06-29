use crate::Settings;
use crate::Vector2;
use crate::Color;

pub fn drawline(pixels: &mut Vec<String>, from: Vector2, to: Vector2, color: Color)
{
	let distance_x = to.x() - from.x();
	let distance_y = to.y() - from.y();
	let mut x = from.x();
	let mut y = from.y();
	let mut p = 2.0 * distance_y - distance_x;

	while x < to.x()
	{
		if p >= 0.0
		{
			let pixel_index = (3.0 + (y * Settings::get_image_width() as f64 + x)) as usize;
			let color_string = Color::write_color(color, 1);
			pixels[pixel_index] = color_string;
			y = y + 1.0;
			p = p + 2.0 * distance_y - 2.0 * distance_x;
		}
		else
		{
			let pixel_index = (3.0 + (y * Settings::get_image_width() as f64 + x)) as usize;
			let color_string = Color::write_color(color, 1);
			pixels[pixel_index] = color_string;
			p = p + 2.0 * distance_y;
		}
		x = x + 1.0;
	}
}

/*
pub fn drawlines(pixels: &mut Vec<String>, lines_data: Vec<(Vector2, Vector2, Color)>)//from: Vector2, to: Vector2, color: Color)
{
	for line_data in lines_data:
	{

	}

	let distance_x = to.x() - from.x();
	let distance_y = to.y() - from.y();
	let mut x = from.x();
	let mut y = from.y();
	let mut p = 2.0 * distance_y - distance_x;

	while x < to.x()
	{
		if p >= 0.0
		{
			let pixel_index = (3.0 + (y * Settings::get_image_width() as f64 + x)) as usize;
			let color_string = Color::write_color(color, 1);
			pixels[pixel_index] = color_string;
			y = y + 1.0;
			p = p + 2.0 * distance_y - 2.0 * distance_x;
		}
		else
		{
			let pixel_index = (3.0 + (y * Settings::get_image_width() as f64 + x)) as usize;
			let color_string = Color::write_color(color, 1);
			pixels[pixel_index] = color_string;
			p = p + 2.0 * distance_y;
		}
		x = x + 1.0;
	}
}
*/