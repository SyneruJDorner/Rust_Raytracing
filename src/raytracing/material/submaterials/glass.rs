use libm::fmin;

use crate::Point;
use crate::Vector3;
use crate::Color;
use crate::Ray;
use crate::HitRecord;
use crate::Scatterable;
use crate::Emmitable;
use crate::random_float;
use crate::Normalable;

#[derive(Debug, Clone, Copy)]
pub struct Glass
{
    pub refract_index: f64
}

impl Glass
{
    #[allow(dead_code)]
    pub fn new(refract_index: f64) -> Glass
    {
        Glass { refract_index: refract_index }
    }
}

impl Scatterable for Glass
{
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>
    {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let direction = hit_record.direction;
        let mut normal = hit_record.normal;
        let mut refraction_ratio = self.refract_index;
        let inside = Vector3::dot(direction, normal) < 0.0;

        if inside
        {
            refraction_ratio = 1.0 / self.refract_index;
            normal = -normal;
        }
        
        let ray_origin: Point = hit_record.hit_point + 0.0001 * normal;
        let cos_theta = fmin(Vector3::dot(direction, normal) as f64, 1.0) as f64;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta < 0.0;

        if reflectance(cos_theta, refraction_ratio) > random_float(0.0, 1.0) || cannot_refract
        {
            //Handle Reflection
            let scatter_direction = Vector3::reflect(direction, normal);
            let scattered_ray = Ray::new(ray_origin, scatter_direction, hit_record.uuid);
            return Some((scattered_ray, attenuation));
        }

        //Handle Refraction
        let scatter_direction = Vector3::refract(direction, -normal, refraction_ratio);
        let scattered_ray = Ray::new(ray_origin, scatter_direction, hit_record.uuid);
        return Some((scattered_ray, attenuation));
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64
{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Emmitable for Glass
{
    #[allow(unused_variables)]
    fn emitted(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(Color::new(0.0, 0.0, 0.0));
    }
}

impl Normalable for Glass
{
    #[allow(unused_variables)]
    fn normals(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(hit_record.normal.to_color());
    }
}