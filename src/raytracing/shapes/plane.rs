use crate::Vector3;
//use crate::AABB;
use crate::Ray;
use crate::Hittable;
use crate::HitRecord;
use crate::Material;
use crate::Lambertian;
use crate::Transform;
use uuid::Uuid;

//use libm::{fmin, fmax};

#[derive(Copy, Debug, Clone)]
pub struct Plane
{
    pub uuid: Uuid,
    pub transform: Transform,
    pub material: Material,
    //pub aabb_bounds: AABB,
}

impl Plane
{
    #[allow(dead_code)]
    pub fn new() -> Plane
    {
        Plane
        {
            uuid: Uuid::new_v4(),
            transform: Transform::new(),
            material: Material::Lambertian(Lambertian::new(1.0, 1.0, 1.0))
            //aabb_bounds: Plane::aabb_bounds(_x0, _x1, _y0, _y1, transform.position.z)
        }
    }

    pub fn calculate_hit(&self, intersection_distance: f64, world_ray: Ray) -> HitRecord
    {
        //Calcaulte the normal of the plane at the intersection point
        let hit_point = world_ray.at(intersection_distance);
        let direction = world_ray.direction.normalize();
        let normal = self.normal_at().normalize();
        return HitRecord::new(self.uuid, intersection_distance, hit_point, direction, normal, self.material);
    }

    pub fn normal_at(&self) -> Vector3
    {
        return Vector3::new(0.0, 1.0, 0.0);
    }
}

impl Hittable for Plane 
{
    #[inline(always)]
    fn hit(&self, world_ray: Ray) -> Option<HitRecord>
    {
        let inverse = self.transform.transform.inverse();

        if inverse.is_none()
        {
            return None;
        }

        let local_ray = world_ray.transform(inverse.unwrap());

        if local_ray.direction.y().abs() < 0.001
        {
            return None;
        }

        let distance = -local_ray.origin.y() / local_ray.direction.y();

        if distance < 0.0
        {
            return None;
        }

        //Determine if the hit is within the bounds of the plane
        let hit_point = local_ray.at(distance);
        if hit_point.x() < 0.0 - (self.transform.scale.x() / 2.0) ||
           hit_point.x() > 0.0 + (self.transform.scale.x() / 2.0) ||
           hit_point.z() < 0.0 - (self.transform.scale.y() / 2.0) ||
           hit_point.z() > 0.0 + (self.transform.scale.y() / 2.0)
        {
            return None;
        }

        return Some(self.calculate_hit(distance, world_ray));
    }

    // fn hit_aabb_bounds(&self, ray: &Ray) -> bool
    // {
    //     let min = self.aabb_bounds.min;
    //     let max = self.aabb_bounds.max;
    //     let mut tmin = -INFINITY;
    //     let mut tmax = INFINITY;

    //     let direction_x = ray.direction.x();
    //     if direction_x > 0.0
    //     {
    //         let tx1 = (min.x - ray.origin.x()) * direction_x;
    //         let tx2 = (max.x - ray.origin.x()) * direction_x;

    //         tmin = fmin(tx1 as f64, tx2 as f64);
    //         tmax = fmax(tx1 as f64, tx2 as f64);
    //     }

    //     let direction_y = ray.direction.y();
    //     if direction_y > 0.0
    //     {
    //         let ty1 = (min.y - ray.origin.y()) * direction_y;
    //         let ty2 = (max.y - ray.origin.y()) * direction_y;

    //         tmin = fmax(tmin, fmin(ty1 as f64, ty2 as f64));
    //         tmax = fmin(tmax, fmax(ty1 as f64, ty2 as f64));
    //     }

    //     let direction_z = ray.direction.z();
    //     if direction_z > 0.0
    //     {
    //         let tz1 = (min.z - ray.origin.z()) * direction_z;
    //         let tz2 = (max.z - ray.origin.z()) * direction_z;

    //         tmin = fmax(tmin as f64, fmin(tz1 as f64, tz2 as f64));
    //         tmax = fmin(tmax as f64, fmax(tz1 as f64, tz2 as f64));
    //     }

    //     return tmax > fmax(tmin, 0.0);
    // }
}
