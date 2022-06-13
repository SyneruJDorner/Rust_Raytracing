use crate::constants::INFINITY;
use crate::vec3::Vec3;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::material::Material;

use libm::{fmin, fmax};

#[derive(Copy, Debug, Clone)]
pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
    pub aabb_bounds: AABB,
}

impl Sphere
{
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere
    {
        Sphere
        {
            center: center,
            radius: radius,
            material: material,
            aabb_bounds:Sphere::aabb_bounds(center, radius)
        }
    }

    fn aabb_bounds(center: Vec3, radius: f32) -> AABB
    {
        let min = center - Vec3::new(radius, radius, radius);
        let max = center + Vec3::new(radius, radius, radius);
        let output_box = AABB::new(&min, &max);
        return output_box;
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
        let u = 0.0;
        let v = 0.0;

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
                        material: self.material,
                        u: u,
                        v: v
                    });
                }
            }
        }
        return None;
    }

    fn hit_aabb_bounds(&self, ray: &Ray) -> bool
    {
        let min = self.aabb_bounds.min;
        let max = self.aabb_bounds.max;
        let mut tmin = -INFINITY;
        let mut tmax = INFINITY;

        let direction_x = ray.direction.x;
        if direction_x > 0.0
        {
            let tx1 = (min.x - ray.origin.x) * direction_x;
            let tx2 = (max.x - ray.origin.x) * direction_x;

            tmin = fmin(tx1 as f64, tx2 as f64);
            tmax = fmax(tx1 as f64, tx2 as f64);
        }

        let direction_y = ray.direction.y;
        if direction_y > 0.0
        {
            let ty1 = (min.y - ray.origin.y) * direction_y;
            let ty2 = (max.y - ray.origin.y) * direction_y;

            tmin = fmax(tmin, fmin(ty1 as f64, ty2 as f64));
            tmax = fmin(tmax, fmax(ty1 as f64, ty2 as f64));
        }

        let direction_z = ray.direction.z;
        if direction_z > 0.0
        {
            let tz1 = (min.z - ray.origin.z) * direction_z;
            let tz2 = (max.z - ray.origin.z) * direction_z;

            tmin = fmax(tmin as f64, fmin(tz1 as f64, tz2 as f64));
            tmax = fmin(tmax as f64, fmax(tz1 as f64, tz2 as f64));
        }

        return tmax > fmax(tmin, 0.0);
    }
}