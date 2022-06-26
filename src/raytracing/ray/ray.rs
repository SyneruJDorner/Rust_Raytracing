use crate::Settings;
use crate::Matrix;
use crate::Point;
use crate::Vector3;
use crate::Color;
use crate::Hittable;
use crate::HittableList;
use crate::{Scatterable, Emmitable, Normalable};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
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
            previous_uuid: previous_uuid
        }
    }

    pub fn at(&self, t: f64) -> Point
    {
        return self.origin + t * self.direction;
    }

    pub fn transform(&self, m: Matrix) -> Ray
    {
        Ray
        {
            origin: m * self.origin,
            direction: m * self.direction,
            color: self.color,
            previous_uuid: self.previous_uuid
        }
    }

    #[allow(dead_code)]
    pub fn calcaulte_ray(ray: Ray, world: &HittableList, depth: u32)  -> Color
    {
        if depth == 0
        {
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit = world.hit(ray);
        
        if hit.is_some()
        {
            let closest_hit = hit.unwrap();
            let scatter = closest_hit.material.scatter(closest_hit);
            let emitted = closest_hit.material.emitted(closest_hit);
            let emmittion = emitted.unwrap();

            if Settings::get_debug_normals() == true
            {
                let normals = closest_hit.material.normals(closest_hit);
                if normals.is_some()
                {
                    let normal_color = normals.unwrap();
                    return normal_color.rgb();
                }
            }

            if scatter.is_some()
            {
                let scattered_ray = scatter.unwrap().0;
                let attenuation = scatter.unwrap().1;
                let scattered = Ray::calcaulte_ray(scattered_ray, world, depth - 1);
                
                return emmittion + attenuation * scattered;
            }

            return emmittion;
        }

        let unit_direction = Vector3::normalize(&ray.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        let color_1 = Color::new(1.0, 1.0, 1.0);
        let color_2 = Color::new(0.5, 0.7, 1.0);
        let sky_color = (1.0 - t) * color_1 + t * color_2;
        return sky_color;
    }
}