use crate::Vector3;
use crate::Ray;
use crate::Hittable;
use crate::HitRecord;
use crate::Material;
use crate::Lambertian;
use crate::Transform;
use uuid::Uuid;

#[derive(Copy, Debug, Clone)]
pub struct Triangle
{
    pub uuid: Uuid,
    pub transform: Transform,
    pub material: Material
}

impl Triangle
{
    #[allow(dead_code)]
    pub fn new() -> Triangle
    {
        Triangle
        {
            uuid: Uuid::new_v4(),
            transform: Transform::new(),
            material: Material::Lambertian(Lambertian::new(1.0, 1.0, 1.0))
        }
    }

    pub fn calculate_hit(&self, intersection_distance: f64, world_ray: Ray) -> HitRecord
    {
        //Calcaulte the normal of the Triangle at the intersection point
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

impl Hittable for Triangle 
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
        let p1 = hit_point.x() < 0.0 - (self.transform.scale.x() / 2.0);
        let p2 = hit_point.x() > 0.0 + (self.transform.scale.x() / 2.0);
        let p3 = hit_point.z() < 0.0 - (self.transform.scale.z() / 2.0);

        //Determine the diagonal of a triangle, then use the hit uv to determine if the hit is within the triangle
        let p4 = hit_point.x() + hit_point.z() > -(self.transform.scale.x() / 2.0) + (self.transform.scale.z() / 2.0);

        if p1 || p2 || p3 || p4
        {
            return None;
        }

        return Some(self.calculate_hit(distance, world_ray));
    }

    fn hit_aabb(&self, world_ray: Ray) -> bool
    {
        return self.transform.aabb_bounds.hit(world_ray);
    }
}
