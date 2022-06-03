/*
bool sphere::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    vec3 oc = r.origin() - center;
    auto a = r.direction().length_squared();
    auto half_b = dot(oc, r.direction());
    auto c = oc.length_squared() - radius*radius;

    auto discriminant = half_b*half_b - a*c;
    if (discriminant < 0) return false;
    auto sqrtd = sqrt(discriminant);

    // Find the nearest root that lies in the acceptable range.
    auto root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root)
            return false;
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    rec.normal = (rec.p - center) / radius;

    return true;
}
*/

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32
}

//Implement the class
impl Sphere
{
    pub fn new(center: Vec3, radius: f32) -> Sphere
    {
        Sphere { center: center, radius: radius }
    }
}

/*
bool sphere::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    vec3 oc = r.origin() - center;
    auto a = r.direction().length_squared();
    auto half_b = dot(oc, r.direction());
    auto c = oc.length_squared() - radius*radius;

    auto discriminant = half_b*half_b - a*c;
    if (discriminant < 0) return false;
    auto sqrtd = sqrt(discriminant);

    // Find the nearest root that lies in the acceptable range.
    auto root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root)
            return false;
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    rec.normal = (rec.p - center) / radius;

    return true;
}
*/

//Implement the class from virtual Hittable
impl Hittable for Sphere 
{
    //implenebt hit function from Hittable
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.at(*root);
                    let mut normal = (p - self.center) / self.radius;
                    let front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
                    
                    if front_face == false
                    {
                        normal = Vec3::inverse(&normal);
                    }


                    return Some(HitRecord {
                        distance: *root,
                        point: p,
                        normal: normal,
                        front_face
                    });
                }
            }
        }
        return None;
    }
}