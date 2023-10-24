use crate::Settings;
use crate::Matrix;
use crate::{Point, Vector3};
use crate::Color;
use crate::{Hittable, HittableList};
use crate::{Scatterable, Emmitable, Normalable};
use crate::{DebugQueue};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Ray
{
    pub origin: Point,
    pub direction: Vector3,
    pub color: Color,
    pub previous_uuid: Uuid
}

impl Ray
{
    pub fn new(origin: Point, direction: Vector3, previous_uuid: Uuid) -> Ray
    {
        Ray
        {
            origin,
            direction,
            color: Color::new(0.0, 0.0, 0.0),
            previous_uuid: previous_uuid,
        }
    }

    pub fn at(&self, distance: f64) -> Point
    {
        return self.origin + distance * self.direction
    }

    pub fn transform(&self, m: &Matrix) -> Ray
    {
        Ray
        {
            origin: m * self.origin,
            direction: m * self.direction,
            color: self.color,
            previous_uuid: self.previous_uuid,
        }
    }

    #[allow(dead_code)]
    pub fn calculate_ray(ray: &Ray, world: &HittableList, depth: u32) -> Color
    {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0).clamp();
        }
    
        if let Some(closest_hit) = world.hit(ray) {
            let material = closest_hit.get_material();
    
            if Settings::get_debug_normals() {
                if let Some(normal_color) = material.normals(closest_hit) {
                    return normal_color.rgb().clamp();
                }
            }
    
            if Settings::get_debug_aabb() {
                let uuid = closest_hit.uuid;
                let vertices = closest_hit.get_hit_transform().aabb_bounds.get_vertices();
                DebugQueue::add_to_debug_queue(uuid, vertices, closest_hit.get_hit_transform().transform);
            }
    
            let emmittion = material.emitted(closest_hit).unwrap_or_default();
    
            if let Some((scattered_ray, attenuation)) = material.scatter(closest_hit) {
                let scattered = Ray::calculate_ray(&scattered_ray, world, depth - 1);
                return (emmittion + attenuation * scattered).clamp();
            }
    
            return emmittion.clamp();
        }
    
        let unit_direction = Vector3::normalize(&ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        let sky_color = ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)).clamp();
        sky_color
    }
}