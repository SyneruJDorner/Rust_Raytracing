use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct HitRecord
{
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool
}

impl HitRecord
{
    #[allow(dead_code)]
    pub fn new() -> HitRecord
    {
        HitRecord { point: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), distance: 0.0, front_face: false }
    }

    #[allow(dead_code)]
    pub fn set_face_normal(mut self, r: &Ray, outward_normal: &Vec3)
    {
        let front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;

        if front_face == true
        {
            self.normal = Vec3::inverse(&outward_normal);
        }
        else
        {
            self.normal = *outward_normal;
        }
    }
}

pub trait Hittable
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}