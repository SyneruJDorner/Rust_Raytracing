use crate::constants::INFINITY;
use crate::vec3::Vec3;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::transform::Transform;

use libm::{fmin, fmax};

#[derive(Copy, Debug, Clone)]
pub struct Plane
{
    pub transform: Transform,
    pub width: f32,
    pub height: f32,
    pub material: Material,
    pub normal: Vec3,
    pub aabb_bounds: AABB,

    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32
}

impl Plane
{
    #[allow(dead_code)]
    pub fn new(transform: Transform, width: f32, height: f32, material: Material) -> Plane
    {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let _x0 = transform.position.x - half_width;
        let _x1 = transform.position.x + half_width;
        let _y0 = transform.position.y - half_height;
        let _y1 = transform.position.y + half_height;

        Plane
        {
            transform: transform,
            width: width,
            height: height,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            material: material,
            normal: Vec3::new(0.0, 0.0, -1.0),
            aabb_bounds: Plane::aabb_bounds(_x0, _x1, _y0, _y1, transform.position.z)
        }
    }

    fn aabb_bounds(x0: f32, x1: f32, y0: f32, y1: f32, z: f32) -> AABB
    {
        let min = Vec3::new(x0, y0, z - 0.0001);
        let max = Vec3::new(x1, y1, z + 0.0001);
        let output_box = AABB::new(&min, &max);
        return output_box;
    }
}

impl Hittable for Plane 
{
    #[inline(always)]
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let distance = (self.transform.position.z - ray.origin.z) / ray.direction.z;
        if distance < t_min || distance > t_max
        {
            return None;
        }

        let x = ray.origin.x + distance * ray.direction.x;
        let y = ray.origin.y + distance * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1
        {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        //let t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mat = self.material;
        let point = ray.at(distance);
        let normal = self.normal;
        let mut hit: HitRecord = HitRecord { point: point, normal: normal, distance: distance, front_face: true, material: mat, u: u, v: v };
        hit.set_face_normal(ray, &outward_normal);
        return Some(hit);
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
