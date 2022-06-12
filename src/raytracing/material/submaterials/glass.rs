use libm::fmin;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::material::Scatterable;
use crate::utils::random_float;

#[derive(Debug, Clone, Copy)]
pub struct Glass
{
    pub refract_index: f32
}

impl Glass
{
    #[allow(dead_code)]
    pub fn new(refract_index: f32) -> Glass
    {
        Glass { refract_index: refract_index }
    }
}

impl Scatterable for Glass
{
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>
    {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio;

        if hit_record.front_face
        {
            refraction_ratio = 1.0 / self.refract_index;
        }
        else
        {
            refraction_ratio = self.refract_index;
        }

        let unit_direction = ray.direction().normalize();
        let cos_theta = fmin(Vec3::dot(&Vec3::inverse(&unit_direction), &hit_record.normal) as f64, 1.0) as f32;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float(0.0, 1.0)
        {
            direction = Vec3::reflect(&unit_direction, &hit_record.normal);
        }
        else
        {
            direction = Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }

        let scattered_ray = Ray::new(hit_record.point, direction);
        return Some((scattered_ray, attenuation));
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32
{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}