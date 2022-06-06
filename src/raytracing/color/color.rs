use crate::vec3::Vec3;
use crate::utils::clamp;

#[allow(dead_code)]
pub fn write_color(rgb: Vec3, samples_per_pixel: i32)
{
    let mut r = rgb.x;
    let mut g = rgb.y;
    let mut b = rgb.z;

    let scale = 1.0 / (samples_per_pixel as f32);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (255.99 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (255.99 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (255.99 * clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}