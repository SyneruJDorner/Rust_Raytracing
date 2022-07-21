use crate::Tuple;
use crate::Vector3;
use crate::Point;
use crate::Ray;
use crate::Hittable;
use crate::{HitRecord, HitInfo, HitObject};
use crate::Material;
use crate::Lambertian;
use crate::Transform;
use uuid::Uuid;
use libm::{fmin, fmax};

#[derive(Copy, Debug, Clone)]
pub struct Sphere
{
    pub uuid: Uuid,
    pub transform: Transform,
    pub material: Material
}

impl Sphere
{
    #[allow(dead_code)]
    pub fn new() -> Sphere
    {
        // let default_material = Material::default();
        let default_material = Material::Lambertian(Lambertian::new(1.0, 1.0, 1.0));
        Sphere
        {
            uuid: Uuid::new_v4(),
            transform: Transform::new(),
            material: default_material
        }
    }

    pub fn calculate_hit(&self, intersection_distance: f64, world_ray: Ray) -> HitRecord
    {
        let hit_point = world_ray.at(intersection_distance);
        let direction = world_ray.direction.normalize();
        let normal = self.normal_at(hit_point).normalize();
        let hit_info = HitInfo::new(intersection_distance, hit_point, direction, normal);
        let hit_object = HitObject::new(self.material, self.transform);
        return HitRecord::new(self.uuid, hit_info, hit_object);
    }

    pub fn normal_at(&self, world_point: Point) -> Vector3
    {
        let inverse = self.transform.transform.inverse().unwrap();
        let local_point = inverse * world_point;
        let local_normal = local_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = Tuple::from(inverse.transpose() * local_normal.to_tuple()).to_vector();
        return world_normal.normalize();
    }
}

impl Hittable for Sphere 
{
    fn hit(&self, world_ray: Ray) -> Option<HitRecord>
    {
        let inverse = self.transform.transform.inverse();

        if inverse.is_none()
        {
            return None;
        }
        
        let local_ray = world_ray.transform(inverse.unwrap());

        let sphere_to_ray = local_ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = Vector3::dot(local_ray.direction, local_ray.direction);
        let b = 2.0 * Vector3::dot(local_ray.direction, sphere_to_ray);
        let c = Vector3::dot(sphere_to_ray, sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0
        {
            return None;
        }

        let distance_1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let distance_2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if fmax(distance_1 as f64, distance_2 as f64) < 0.0
        {
            return None;
        }

        let mut closest_hit = fmin(distance_1 as f64, distance_2 as f64) as f64;

        if closest_hit <= 0.0
        {
            closest_hit = fmax(distance_1 as f64, distance_2 as f64) as f64;
        }

        return Some(self.calculate_hit(closest_hit, world_ray));
    }

    fn hit_aabb(&self, world_ray: Ray) -> bool
    {
        return self.transform.aabb_bounds.hit(world_ray);
    }
}