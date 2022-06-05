use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::material::Material;

#[derive(Copy, Debug, Clone)]
pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Sphere
{
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere
    {
        Sphere { center: center, radius: radius, material: material }
    }
}

impl Hittable for Sphere 
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let distance_a = ((-half_b) - sqrtd) / a;
            let distance_b = ((-half_b) + sqrtd) / a;
            for distance in [distance_a, distance_b].iter() {
                if *distance < t_max && *distance > t_min {
                    let point = ray.at(*distance);
                    let mut normal = (point - self.center) / self.radius;
                    let front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
                    
                    if front_face == false
                    {
                        normal = Vec3::inverse(&normal);
                    }

                    return Some(HitRecord {
                        distance: *distance,
                        point: point,
                        normal: normal,
                        front_face: front_face,
                        material: self.material
                    });
                }
            }
        }
        return None;
    }
}