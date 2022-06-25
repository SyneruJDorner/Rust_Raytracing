//use crate::constant::INFINITY;
use crate::EPSILON;
use crate::Tuple;
use crate::Vector3;
//use crate::AABB;
use crate::Ray;
use crate::Hittable;
use crate::HitRecord;
use crate::Material;
use crate::Transform;
use uuid::Uuid;

//use libm::{fmin, fmax};

#[derive(Copy, Debug, Clone)]
pub struct Plane
{
    pub uuid: Uuid,
    pub transform: Transform,
    // pub width: f64,
    // pub height: f64,
    pub material: Material,
    //pub aabb_bounds: AABB,

    // x0: f64,
    // x1: f64,
    // y0: f64,
    // y1: f64
}

impl Plane
{
    #[allow(dead_code)]
    pub fn new(transform: Transform, material: Material) -> Plane
    {
        // let object_potsition = Point::new(0.0, 0.0, 0.0);
        // let half_width = width / 2.0;
        // let half_height = height / 2.0;
        // let _x0 = object_potsition.x() - half_width;
        // let _x1 = object_potsition.x() + half_width;
        // let _y0 = object_potsition.y() - half_height;
        // let _y1 = object_potsition.y() + half_height;

        Plane
        {
            uuid: Uuid::new_v4(),
            transform: transform,
            // width: width,
            // height: height,
            // x0: _x0,
            // x1: _x1,
            // y0: _y0,
            // y1: _y1,
            material: material,
            //aabb_bounds: Plane::aabb_bounds(_x0, _x1, _y0, _y1, transform.position.z)
        }
    }

    pub fn normal_at(&self) -> Vector3
    {
        let inverse = self.transform.transform.inverse().unwrap();
        let local_normal = Vector3::new(0.0, 1.0, 0.0);
        let normal = inverse.transpose() * local_normal;
        let world_normal = Tuple::from(normal.to_tuple());
        let world_normal = Vector3::new(world_normal.x(), world_normal.y(), world_normal.z());
        world_normal.normalize()
    }

    pub fn calculate_hit(&self, intersection: f64, world_ray: Ray) -> HitRecord
    {
        let world_point = world_ray.at_local(intersection);
        let distance = intersection;
        let inverted_dir = -world_ray.direction;
        let normal = self.normal_at();
        return HitRecord::new(self.uuid, distance, world_point, inverted_dir, normal, self.material);
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

        if local_ray.direction.y().abs() < EPSILON
        {
            return None;
        }

        let distance = -local_ray.origin.y() / local_ray.direction.y();
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
