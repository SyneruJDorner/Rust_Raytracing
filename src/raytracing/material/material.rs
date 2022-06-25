use crate::Color;
use crate::Ray;
use crate::HitRecord;

pub mod submaterials
{
    pub mod lambertian;
    pub mod metal;
    pub mod glass;
    pub mod emmision;
}


pub trait Scatterable
{
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>;
}

pub trait Emmitable
{
    fn emitted(&self, hit_record: HitRecord) ->  Option<Color>;
}

pub trait Normalable
{
    fn normals(&self, hit_record: HitRecord) -> Option<Color>;
}

#[derive(Copy, Clone, Debug)]
pub enum Material
{
    #[allow(dead_code)]
    Lambertian(submaterials::lambertian::Lambertian),
    #[allow(dead_code)]
    Metal(submaterials::metal::Metal),
    #[allow(dead_code)]
    Glass(submaterials::glass::Glass),
    #[allow(dead_code)]
    Emmition(submaterials::emmision::Emmision)
}

impl Scatterable for Material
{
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>
    {
        match self
        {
            Material::Lambertian(l) => l.scatter(hit_record),
            Material::Metal(m) => m.scatter(hit_record),
            Material::Glass(g) => g.scatter(hit_record),
            Material::Emmition(e) => e.scatter(hit_record)
        }
    }
}

impl Emmitable for Material
{
    fn emitted(&self, hit_record: HitRecord) -> Option<Color>
    {
        match self
        {
            Material::Lambertian(l) => l.emitted(hit_record),
            Material::Metal(m) => m.emitted(hit_record),
            Material::Glass(g) => g.emitted(hit_record),
            Material::Emmition(e) => e.emitted(hit_record)
        }
    }
}

impl Normalable for Material
{
    fn normals(&self, hit_record: HitRecord) -> Option<Color>
    {
        match self
        {
            Material::Lambertian(l) => l.normals(hit_record),
            Material::Metal(m) => m.normals(hit_record),
            Material::Glass(g) => g.normals(hit_record),
            Material::Emmition(e) => e.normals(hit_record)
        }
    }
}