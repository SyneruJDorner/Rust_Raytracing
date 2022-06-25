use crate::Vector3;
use crate::Point;
use crate::Ray;
use crate::Material;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord
{
    pub uuid: Uuid,
    pub distance: f64,
    
    pub hit_point: Point,
    pub direction: Vector3,
    pub normal: Vector3,

    pub material: Material
}

impl HitRecord
{
    pub fn new(uuid: Uuid, distance: f64, hit_point: Point, direction: Vector3, normal: Vector3, material: Material) -> HitRecord
    {
        return HitRecord
        {
            uuid: uuid,
            hit_point: hit_point,
            distance: distance,
            direction: direction,
            normal: normal,

            material: material
        };
    }
}

pub trait Hittable
{
    fn hit(&self, world_ray: Ray) -> Option<HitRecord>;
    //fn hit_aabb_bounds(&self, ray: &Ray) -> bool;
}