
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::material::Scatterable;
use crate::material::Emmitable;

#[derive(Debug, Clone, Copy)]
pub struct Emmision
{
    pub emit: Vec3
}

impl Emmision
{
    #[allow(dead_code)]
    pub fn new(color: Vec3) -> Emmision
    {
        Emmision
        {
            emit: color
        }
    }
}

impl Scatterable for Emmision
{
    #[allow(unused_variables)]
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        return None;
    }
}

impl Emmitable for Emmision
{
    #[allow(unused_variables)]
    fn emitted(&self, ray: &Ray, hit_record: &HitRecord) -> Vec3
    {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}