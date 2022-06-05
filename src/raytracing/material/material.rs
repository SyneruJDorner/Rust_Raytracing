use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub mod submaterials
{
    pub mod lamertian;
    pub mod metal;
}

pub trait Scatterable
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

#[derive(Copy, Clone, Debug)]
pub enum Material
{
    Lambertian(submaterials::lamertian::Lambertian),
    Metal(submaterials::metal::Metal)
}

impl Scatterable for Material
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        match self
        {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record)
        }
    }
}

