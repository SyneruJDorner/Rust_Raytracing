use uuid::Uuid;
use crate::{Vector3, Ray, AABB, Transform};
use crate::{Hittable, HitRecord, HitInfo, HitObject};
use crate::{Material, Lambertian};

#[derive(Copy, Debug, Clone)]
pub struct Plane
{
    pub uuid: Uuid,
    pub transform: Transform,
    pub material: Material
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
        }
    }

    pub fn calculate_hit(&self, intersection_distance: f64, world_ray: &Ray) -> HitRecord
    {
        //Calcalute the normal of the plane at the intersection point
        let hit_point = world_ray.at(intersection_distance);
        let direction = world_ray.direction.normalize();
        let normal = self.normal_at().normalize();
        let hit_info = HitInfo::new(intersection_distance, hit_point, direction, normal);
        let hit_object = HitObject::new(self.material, self.transform);
        return HitRecord::new(self.uuid, hit_info, hit_object);
    }

    pub fn normal_at(&self) -> Vector3
    {
        let local_normal = Vector3::new(0.0, 1.0, 0.0); // Local space normal
        let world_normal = self.transform.transform_normal(&local_normal).unwrap(); // Transform the normal to world space
        return world_normal.normalize();
    }
}

impl Hittable for Plane 
{
    #[inline(always)]
    fn hit(&self, world_ray: &Ray) -> Option<HitRecord>
    {
        let inverse = self.transform.transform.inverse();

        if inverse.is_none()
        {
            return None;
        }

        let local_ray = world_ray.transform(&inverse.unwrap());

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
        let p1 = hit_point.x() < 0.0 - (self.transform.scale.x() / 2.0);
        let p2 = hit_point.x() > 0.0 + (self.transform.scale.x() / 2.0);
        let p3 = hit_point.z() < 0.0 - (self.transform.scale.z() / 2.0);
        let p4 = hit_point.z() > 0.0 + (self.transform.scale.z() / 2.0);
        if p1 || p2 || p3 || p4
        {
            return None;
        }

        return Some(self.calculate_hit(distance, world_ray));
    }

    fn hit_aabb(&self, world_ray: &Ray) -> bool
    {
        return self.transform.aabb_bounds.hit(world_ray);
    }

    fn get_aabb(&self) -> AABB
    {
        return self.transform.aabb_bounds;
    }

    fn get_transform(&self) -> Transform
    {
        return self.transform;
    }
}
