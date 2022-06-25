use crate::Vector3;
use crate::Color;
use crate::Ray;
use crate::HitRecord;
use crate::Scatterable;
use crate::Emmitable;
use crate::Normalable;

#[derive(Debug, Clone, Copy)]
pub struct Metal
{
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal
{
    #[allow(dead_code)]
    pub fn new(r: f64, g: f64, b: f64, mut fuzz: f64) -> Metal
    {
        if fuzz > 1.0
        {
            fuzz = 1.0;
        }

        Metal
        {
            albedo: Color::new(r, g, b),
            fuzz: fuzz
        }
    }

    #[allow(dead_code)]
    pub fn set(albedo: Color, mut fuzz: f64) -> Metal
    {
        if fuzz > 1.0
        {
            fuzz = 1.0;
        }
        Metal { albedo: albedo, fuzz: fuzz }
    }
}

impl Scatterable for Metal
{
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>
    {
        let reflected = Vector3::reflect(hit_record.direction, hit_record.normal);
        let scattered_ray = Ray::new(hit_record.hit_point, reflected + self.fuzz * Vector3::random_in_unit_sphere(), hit_record.uuid);
        let attenuation = self.albedo;
        if Vector3::dot(scattered_ray.direction, hit_record.normal) > 0.0
        {
            return Some((scattered_ray, attenuation));
        }
        else
        {
            return None;
        }
    }
}

impl Emmitable for Metal
{
    #[allow(unused_variables)]
    fn emitted(&self, hit_record: HitRecord) ->  Option<Color>
    {
        return Some(Color::new(0.0, 0.0, 0.0));
    }
}

impl Normalable for Metal
{
    #[allow(unused_variables)]
    fn normals(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(hit_record.normal.to_color());
    }
}