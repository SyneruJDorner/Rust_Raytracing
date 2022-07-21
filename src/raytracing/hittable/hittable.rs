use crate::{Vector3, Point};
use crate::Transform;
use crate::Ray;
use crate::Material;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo
{
    pub distance: f64,
    pub hit_point: Point,
    pub direction: Vector3,
    pub normal: Vector3,
}

impl HitInfo
{
    pub fn new(distance: f64, hit_point: Point, direction: Vector3, normal: Vector3) -> HitInfo
    {
        return HitInfo
        {
            distance: distance,
            hit_point: hit_point,
            direction: direction,
            normal: normal,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitObject
{
    pub material: Material,
    pub hit_transform: Transform
}

impl HitObject
{
    pub fn new(material: Material, hit_transform: Transform) -> HitObject
    {
        return HitObject
        {
            material: material,
            hit_transform: hit_transform
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord
{
    pub uuid: Uuid,
    
    pub hit_info: HitInfo,
    pub hit_object: HitObject
}

impl HitRecord
{
    pub fn new(uuid: Uuid, hit_info: HitInfo, hit_object: HitObject) -> HitRecord//, distance: f64, hit_point: Point, direction: Vector3, normal: Vector3, transform: Transform, material: Material) -> HitRecord
    {
        return HitRecord
        {
            uuid: uuid,
            hit_info: hit_info,
            hit_object: hit_object
        };
    }

    pub fn get_distance(&self) -> f64
    {
        return self.hit_info.distance;
    }

    pub fn get_hit_point(&self) -> Point
    {
        return self.hit_info.hit_point;
    }

    pub fn get_direction(&self) -> Vector3
    {
        return self.hit_info.direction;
    }

    pub fn get_normal(&self) -> Vector3
    {
        return self.hit_info.normal;
    }

    pub fn get_material(&self) -> Material
    {
        return self.hit_object.material;
    }

    pub fn get_hit_transform(&self) -> Transform
    {
        return self.hit_object.hit_transform;
    }
}

pub trait Hittable
{
    fn hit(&self, world_ray: Ray) -> Option<HitRecord>;
    fn hit_aabb(&self, world_ray: Ray) -> bool;
}