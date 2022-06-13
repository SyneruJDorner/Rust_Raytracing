use crate::vec3::Vec3;
use crate::ray::Ray;

use libm::{fmin, fmax};

//Axis-Aligned Bounding Box
#[derive(Copy, Clone, Debug)]
pub struct AABB
{
    pub min: Vec3,
    pub max: Vec3
}

impl AABB
{
    // #[allow(dead_code)]
    // pub fn new() -> AABB
    // {
    //     return AABB
    //     {
    //         minimum: Vec3::new(0.0, 0.0, 0.0),
    //         maximum: Vec3::new(0.0, 0.0, 0.0)
    //     }
    // }

    #[allow(dead_code)]
    pub fn new(min: &Vec3, max: &Vec3) -> AABB
    {
        return AABB
        {
            min: Vec3::new(min.x, min.y, min.z),
            max: Vec3::new(max.x, max.y, max.z)
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn min(&self) -> Vec3
    {
        return self.min;
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn max(&self) -> Vec3
    {
        return self.max;
    }

    //Inlining this function is a huge performance boost
    #[allow(dead_code)]
    #[inline(always)]
    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> Option<(Option<f32>, Option<f32>)>
    {
        //Compare X
        let inv_d = 1.0 / r.direction.x;
        let mut t0 = (self.min.x - r.origin.x) * inv_d;
        let mut t1 = (self.min.x - r.origin.x) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return Some((None, None)); }

        //Compare Y
        let inv_d = 1.0 / r.direction.y;
        let mut t0 = (self.min.y - r.origin.y) * inv_d;
        let mut t1 = (self.min.y - r.origin.y) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return Some((None, None)); }

        //Compare Z
        let inv_d = 1.0 / r.direction.z;
        let mut t0 = (self.min.z - r.origin.z) * inv_d;
        let mut t1 = (self.min.z - r.origin.z) * inv_d;
        if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1); }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return Some((None, None)); }

        Some((Some(t_min), Some(t_max)))
    }

    #[allow(dead_code)]
    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB
    {
        let small: Vec3 = Vec3::new(fmin(box0.min().x as f64, box1.min().x as f64) as f32, fmin(box0.min().y as f64, box1.min().y as f64) as f32, fmin(box0.min().z as f64, box1.min().z as f64) as f32);
        let big: Vec3 = Vec3::new(fmax(box0.max().x as f64, box1.max().x as f64) as f32, fmax(box0.max().y as f64, box1.max().y as f64) as f32, fmax(box0.max().z as f64, box1.max().z as f64) as f32);
        return AABB::new(&small, &big);
    }
}
