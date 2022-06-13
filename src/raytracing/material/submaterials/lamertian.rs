use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::material::Scatterable;
use crate::material::Emmitable;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian
{
    pub albedo: Vec3
}

impl Lambertian
{
    #[allow(dead_code)]
    pub fn new(albedo: Vec3) -> Lambertian
    {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian
{
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere();

        if scatter_direction.near_zero()
        {
            scatter_direction = hit_record.normal;
        }

        let target = hit_record.point + scatter_direction;
        let scattered_ray = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;
        return Some((scattered_ray, attenuation));
    }
}

impl Emmitable for Lambertian
{
    #[allow(unused_variables)]
    fn emitted(&self, ray: &Ray, hit_record: &HitRecord) -> Vec3
    {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}