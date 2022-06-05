use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::material::Scatterable;

#[derive(Debug, Clone, Copy)]
pub struct Metal
{
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Metal
{
    #[allow(dead_code)]
    pub fn new(albedo: Vec3, mut fuzz: f32) -> Metal
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        let reflected = Vec3::reflect(&ray.direction().normalize(), &hit_record.normal);
        let scattered_ray = Ray::new(hit_record.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if Vec3::dot(&scattered_ray.direction, &hit_record.normal) > 0.0
        {
            return Some((scattered_ray, attenuation));
        }
        else
        {
            return None;
        }
    }
}