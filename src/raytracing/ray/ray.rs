use crate::vec3::Vec3;
use crate::hittablelist::HittableList;
use crate::constants::INFINITY;
use crate::material::Scatterable;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3,
    pub color: Vec3
}

impl Ray
{
    #[allow(dead_code)]
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray { origin: origin, direction: direction, color: Vec3::zero() }
    }

    pub fn origin(self) -> Vec3
    {
        return self.origin;
    }

    pub fn direction(self) -> Vec3
    {
        return self.direction;
    }

    pub fn at(self, t: f32) -> Vec3
    {
        return self.origin + t * self.direction;
    }

    pub fn calcaulte_ray(r: &Ray, world: &HittableList, depth: i32)  -> Vec3
    {
        if depth <= 0
        {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        
        let closest_object = world.hit_closest_object(r, 0.001, INFINITY as f32);
        
        //Handle all reflections
        if closest_object.is_some()
        {
            let hit_obj = closest_object.unwrap();
            let scatter = hit_obj.material.scatter(&r, &hit_obj);
            if scatter.is_some()
            {
                let scattered = scatter.unwrap().0;
                let attenuation = scatter.unwrap().1;
                return attenuation * Ray::calcaulte_ray(&scattered, world, depth - 1);
            }
            return Vec3::new(1.0, 1.0, 1.0);
        }

        let unit_direction = Vec3::normalize(&r.direction());
        let t = 0.5*(unit_direction.y + 1.0);
        let color_1 = Vec3::new(1.0, 1.0, 1.0);
        let color_2 = Vec3::new(0.5, 0.7, 1.0);
        let sky_color = (1.0 - t) * color_1 + t * color_2;
        return sky_color;
    }
}