use crate::Vector3;
use crate::Color;
use crate::Ray;
use crate::HitRecord;
use crate::Scatterable;
use crate::Emmitable;
use crate::Normalable;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian
{
    pub albedo: Color
}

impl Lambertian
{
    #[allow(dead_code)]
    pub fn new(r: f64, g: f64, b: f64) -> Lambertian
    {
        Lambertian
        {
            albedo: Color::new(r, g, b)
        }
    }

    #[allow(dead_code)]
    pub fn from_color(color: Color) -> Lambertian
    {
        Lambertian
        {
            albedo: color
        }
    }
}

impl Scatterable for Lambertian
{
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>
    {
        let mut scatter_direction = hit_record.normal + Vector3::random_in_unit_sphere();
        
        if scatter_direction.near_zero()
        {
            scatter_direction = hit_record.normal;
        }
        
        let target = hit_record.hit_point + scatter_direction;
        let direction = target - hit_record.hit_point;
        let scattered_ray = Ray::new(hit_record.hit_point, direction, hit_record.uuid);
        let attenuation = self.albedo;
        return Some((scattered_ray, attenuation));
    }
}

impl Emmitable for Lambertian
{
    #[allow(unused_variables)]
    fn emitted(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(Color::new(0.0, 0.0, 0.0));
    }
}

impl Normalable for Lambertian
{
    #[allow(unused_variables)]
    fn normals(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(hit_record.normal.to_color());
    }
}