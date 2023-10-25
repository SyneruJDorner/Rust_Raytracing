use crate::{Vector3, Point};
use crate::Ray;
use crate::Transform;
use crate::INFINITY;
use libm::{fmin, fmax};

//Axis-Aligned Bounding Box
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB
{
    pub min: Vector3,
    pub max: Vector3,
    pub aabb_vertices: [Point; 8]
}

impl AABB
{
    #[allow(dead_code)]
    pub fn new() -> AABB
    {
        let min = Vector3::new(-0.5, -0.5, -0.5);
        let max = Vector3::new(0.5, 0.5, 0.5);
        let aabb_vertices = [
            Point::new(min.x(), min.y(), min.z()),
            Point::new(max.x(), min.y(), min.z()),
            Point::new(max.x(), max.y(), min.z()),
            Point::new(min.x(), max.y(), min.z()),
            Point::new(min.x(), min.y(), max.z()),
            Point::new(max.x(), min.y(), max.z()),
            Point::new(max.x(), max.y(), max.z()),
            Point::new(min.x(), max.y(), max.z())
        ];

        return AABB
        {
            min: min,
            max: max,
            aabb_vertices: aabb_vertices
        }
    }

    #[allow(dead_code)]
    pub fn set(min: Vector3, max: Vector3) -> AABB
    {
        return AABB
        {
            min: Vector3::new(min.x(), min.y(), min.z()),
            max: Vector3::new(max.x(), max.y(), max.z()),
            aabb_vertices: [
                Point::new(min.x(), min.y(), min.z()),
                Point::new(max.x(), min.y(), min.z()),
                Point::new(max.x(), max.y(), min.z()),
                Point::new(min.x(), max.y(), min.z()),
                Point::new(min.x(), min.y(), max.z()),
                Point::new(max.x(), min.y(), max.z()),
                Point::new(max.x(), max.y(), max.z()),
                Point::new(min.x(), max.y(), max.z())
            ]
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn min(&self) -> Vector3
    {
        return self.min;
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn max(&self) -> Vector3
    {
        return self.max;
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_vertices(&self) -> [Point; 8]
    {
        return self.aabb_vertices;
    }

    pub fn calcalute_aabb_bounds(transform: &Transform) -> AABB
    {
        let min_x = transform.position.x() - transform.scale.x();
        let max_x = transform.position.x() + transform.scale.x();
        let min_y = transform.position.y() - transform.scale.y();
        let max_y = transform.position.y() + transform.scale.y();
        let min_z = transform.position.z() - transform.scale.z();
        let max_z = transform.position.z() + transform.scale.z();

        let output_box = AABB::set(Vector3::new(min_x, min_y, min_z), Vector3::new(max_x, max_y, max_z));
        return output_box;
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn hit(&self, world_ray: &Ray) -> bool
    {
        let min = self.min;
        let max = self.max;
        let mut tmin = -INFINITY;
        let mut tmax = INFINITY;

        let direction_x = world_ray.direction.x();
        let tx1 = fmin((min.x() - world_ray.origin.x()) / direction_x, (max.x() - world_ray.origin.x()) / direction_x);
        let tx2 = fmax((max.x() - world_ray.origin.x()) / direction_x, (min.x() - world_ray.origin.x()) / direction_x);
        tmin = fmax(tx1, tmin);
        tmax = fmin(tx2, tmax);

        let direction_y = world_ray.direction.y();
        let ty1 = fmin((min.y() - world_ray.origin.y()) / direction_y, (max.y() - world_ray.origin.y()) / direction_y);
        let ty2 = fmax((max.y() - world_ray.origin.y()) / direction_y, (min.y() - world_ray.origin.y()) / direction_y);
        tmin = fmax(ty1, tmin);
        tmax = fmin(ty2, tmax);

        let direction_z = world_ray.direction.z();
        let tz1 = fmin((min.z() - world_ray.origin.z()) / direction_z, (max.z() - world_ray.origin.z()) / direction_z);
        let tz2 = fmax((max.z() - world_ray.origin.z()) / direction_z, (min.z() - world_ray.origin.z()) / direction_z);
        tmin = fmax(tz1, tmin);
        tmax = fmin(tz2, tmax);

        let result = tmax > fmax(tmin, 0.0);
        return result;
    }
}