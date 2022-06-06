use libm::fabs;

mod operators {
    pub mod mul;
    pub mod div;
    pub mod add;
    pub mod sub;
}

use crate::utils::random_float;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3
{
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3
    {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn normalize(&self) -> Vec3
    {
        return self / self.length()
    }

    pub fn length(&self) -> f32
    {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f32
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32
    {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3
    {
        return Vec3 {   x: a.y * b.z - a.z * b.y,
                        y: a.z * b.x - a.x * b.z,
                        z: a.x * b.y - a.y * b.x
        };
    }

    pub fn inverse(a: &Vec3) -> Vec3
    {
        return Vec3 { x: -a.x, y: -a.y, z: -a.z }
    }

    pub fn distance(a: &Vec3, b: &Vec3) -> f32
    {
        return (a - b).length();
    }

    pub fn random(min: f32, max: f32) -> Vec3
    {
        return Vec3 { x: random_float(min, max), y: random_float(min, max), z: random_float(min, max) };
    }

    pub fn random_in_unit_sphere() -> Vec3
    {
        loop
        {
            let random_point = Vec3::random(-1.0, 1.0);
            if random_point.length_squared() >= 1.0
            {
                continue;
            }
            return random_point;
        }
    }

    pub fn random_unit_vector() -> Vec3
    {
        return Vec3::random_in_unit_sphere().normalize();
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3
    {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 // In the same hemisphere as the normal
        {
            return in_unit_sphere;
        }
        else
        {
            return Vec3::new(-in_unit_sphere.x, -in_unit_sphere.y, -in_unit_sphere.z);
        }
    }

    pub fn near_zero(&self) -> bool
    {
        let s = 1e-8;
        return (fabs(self.x as f64) < s) && (fabs(self.y as f64) < s) && (fabs(self.z as f64) < s);
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3
    {
        return v - 2.0 * Vec3::dot(v, n) * n;
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3
    {
        let inverse_uv = Vec3::inverse(uv);
        let uv_val = *uv;
        let n_val = *n;
        let cos_theta = (Vec3::dot(&inverse_uv, n)).min(1.0);
        let r_out_perp = (uv_val + n_val * cos_theta) * etai_over_etat;
        let r_out_parallel = n_val * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());
        return r_out_perp + r_out_parallel
    }
}
