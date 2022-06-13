use crate::matrix4x4::Matrix4x4;
use crate::vec3::Vec3;
use crate::hittablelist::HittableList;
use crate::constants::INFINITY;
use crate::material::Scatterable;
use crate::material::Emmitable;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub dir_matrix: Matrix4x4,
}

impl Ray
{
    #[allow(dead_code)]
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray { origin: origin, direction: direction, color: Vec3::zero(), dir_matrix: Matrix4x4::identity() }
    }

    pub fn origin(&self) -> Vec3
    {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3
    {
        return self.direction;
    }

    pub fn at(&self, t: f32) -> Vec3
    {
        return self.origin + t * self.direction;
    }

    pub fn set_dir_matrix(&mut self, dir_matrix: Matrix4x4)
    {
        self.dir_matrix = dir_matrix;
    }

    pub fn calcaulte_ray(ray: &Ray, background: &Vec3, world: &HittableList, depth: i32)  -> Vec3
    {
        if depth <= 0
        {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        
        let closest_object = world.hit_closest_object(ray, 0.001, INFINITY as f32);
        
        //Handle all reflections
        if closest_object.is_some()
        {
            let hit_obj = closest_object.unwrap();
            let scatter = hit_obj.material.scatter(&ray, &hit_obj);
            let emitted = hit_obj.material.emitted(&ray, &hit_obj);
            if scatter.is_some()
            {
                let scattered = scatter.unwrap().0;
                let attenuation = scatter.unwrap().1;
                // let emitted = rec.mat_ptr->emitted(rec.u, rec.v, rec.p);
                
                return emitted + attenuation * Ray::calcaulte_ray(&scattered, background, world, depth - 1);
            }
            
            //return emitted;
            return Vec3::new(1.0, 1.0, 1.0);
        }

        return *background;

        // let unit_direction = Vec3::normalize(&ray.direction());
        // let t = 0.5 * (unit_direction.y + 1.0);
        // let color_1 = Vec3::new(1.0, 1.0, 1.0);
        // let color_2 = Vec3::new(0.5, 0.7, 1.0);
        // let sky_color = (1.0 - t) * color_1 + t * color_2;
        // return sky_color;
    }
}