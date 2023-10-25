use crate::Settings;
use crate::Matrix;
use crate::{Point, Vector3};
use crate::Color;
use crate::{Hittable, HittableList};
use crate::{Scatterable, Emmitable, Normalable};
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
        if depth == 0
        {
            return Color::new(0.0, 0.0, 0.0).clamp();
        }

        let mut aabb_color: Option<Color> = None;

        if Settings::get_debug_aabb()
        {
            if world.hit_aabb(ray) == true
            {
                aabb_color = Some(Color::new(0.0, 1.0, 0.0).rgb().clamp());
                //return aabb_color.rgb().clamp();
            }
        }

        if let Some(closest_hit) = world.hit(ray)
        {
            let material = closest_hit.get_material();
    
            if Settings::get_debug_normals()
            {
                if let Some(normal_color) = material.normals(closest_hit)
                {
                    return normal_color.rgb().clamp();
                }
            }
    
            let emmittion = material.emitted(closest_hit).unwrap_or_default();
    
            if let Some((scattered_ray, attenuation)) = material.scatter(closest_hit)
            {
                let scattered = Ray::calculate_ray(&scattered_ray, world, depth - 1);
                let final_Color = emmittion + attenuation * scattered;

                // if aabb_color is None
                if aabb_color.is_some()
                {
                    return (final_Color * aabb_color.unwrap()).clamp();
                }
                
                return (final_Color).clamp();
            }
    
            return emmittion.clamp();
        }
    
        if aabb_color.is_some()
        {
            return aabb_color.unwrap();
        }
        
        let unit_direction = Vector3::normalize(&ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        let sky_color = ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)).clamp();
        return sky_color
    }
}