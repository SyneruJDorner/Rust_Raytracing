use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub mod submaterials
{
    pub mod lamertian;
    pub mod metal;
    pub mod glass;
    pub mod emmision;
}


pub trait Scatterable
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub trait Emmitable
{
    fn emitted(&self, ray: &Ray, hit_record: &HitRecord) -> Vec3;//u: f32, v: f32, p: Vec3) -> Vec3;
}

#[derive(Copy, Clone, Debug)]
pub enum Material
{
    #[allow(dead_code)]
    Lambertian(submaterials::lamertian::Lambertian),
    #[allow(dead_code)]
    Metal(submaterials::metal::Metal),
    #[allow(dead_code)]
    Glass(submaterials::glass::Glass),
    #[allow(dead_code)]
    Emmition(submaterials::emmision::Emmision)
}

impl Scatterable for Material
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        match self
        {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Glass(g) => g.scatter(ray, hit_record),
            Material::Emmition(e) => e.scatter(ray, hit_record)
        }
    }
}

impl Emmitable for Material
{
    fn emitted(&self, ray: &Ray, hit_record: &HitRecord) -> Vec3
    {
        match self
        {
            Material::Lambertian(l) => l.emitted(ray, hit_record),
            Material::Metal(m) => m.emitted(ray, hit_record),
            Material::Glass(g) => g.emitted(ray, hit_record),
            Material::Emmition(e) => e.emitted(ray, hit_record)
        }
    }
}
