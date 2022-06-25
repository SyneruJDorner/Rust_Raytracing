use libm::fabs;
use crate::Tuple;
use crate::Color;
use crate::random_float;
use crate::PI;
use crate::Point;

mod operators
{
    pub mod mul;
    pub mod div;
    pub mod add;
    pub mod sub;
    pub mod neg;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3(Tuple);

impl Vector3
{
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3
    {
        return Vector3(Tuple::new(x, y, z, 0.0));
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64
    {
        return self.0.x;
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64
    {
        return self.0.y;
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64
    {
        return self.0.z;
    }

    #[allow(dead_code)]
    pub fn w(&self) -> f64
    {
        return self.0.w;
    }

    #[allow(dead_code)]
    pub fn set_x(&mut self, x: f64)
    {
        self.0.x = x;
    }

    #[allow(dead_code)]
    pub fn set_y(&mut self, y: f64)
    {
        self.0.y = y;
    }

    #[allow(dead_code)]
    pub fn set_z(&mut self, z: f64)
    {
        self.0.z = z;
    }

    #[allow(dead_code)]
    pub fn set_w(&mut self, w: f64)
    {
        self.0.w = w;
    }

    #[allow(dead_code)]
    pub fn to_point(&self) -> Point
    {
        return Point::new(self.x(), self.y(), self.z());
    }

    #[allow(dead_code)]
    pub fn to_tuple(&self) -> Tuple
    {
        return Tuple::new(self.x(), self.y(), self.z(), self.w());
    }

    #[allow(dead_code)]
    pub fn equal_approx(&self, other: Vector3) -> bool
    {
        return self.0.equal_approx(other.0);
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64
    {
        return (self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)).sqrt();
    }

    #[allow(dead_code)]
    pub fn normalize(&self) -> Vector3
    {
        return Vector3(self.0 / self.magnitude());
    }

    #[allow(dead_code)]
    pub fn dot(a: Vector3, other: Vector3) -> f64
    {
        return a.x() * other.x() + a.y() * other.y() + a.z() * other.z() + a.w() * other.w();
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: Vector3) -> Vector3
    {
        return Vector3::new(self.y() * other.z() - self.z() * other.y(), self.z() * other.x() - self.x() * other.z(), self.x() * other.y() - self.y() * other.x());
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f64
    {
        return self.length_squared().sqrt();
    }

    #[allow(dead_code)]
    pub fn length_squared(&self) -> f64
    {
        return self.x() * self.x() + self.y() * self.y() + self.z() * self.z();
    }

    #[allow(dead_code)]
    pub fn reflect(v: Vector3, normal: Vector3) -> Vector3
    {
        return v - 2.0 * Vector3::dot(v, normal) * normal;
    }

    #[allow(dead_code)]
    pub fn refract(direction: Vector3, normal: Vector3, etai_over_etat: f64) -> Vector3
    {
        let inverse_direction = Vector3::inverse(direction);
        let cos_theta = Vector3::dot(inverse_direction, normal).min(1.0);
        let r_out_perp = (direction + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
        return r_out_perp + r_out_parallel
    }

    #[allow(dead_code)]
    pub fn near_zero(&self) -> bool
    {
        let s = 0.0001;
        return (fabs(self.x() as f64) < s) && (fabs(self.y() as f64) < s) && (fabs(self.z() as f64) < s);
    }

    #[allow(dead_code)]
    pub fn inverse(a: Vector3) -> Vector3
    {
        return Vector3::new(-a.x(), -a.y(), -a.z());
    }

    #[allow(dead_code)]
    pub fn random(min: f64, max: f64) -> Vector3
    {
        return Vector3::new(random_float(min, max), random_float(min, max), random_float(min, max));
    }

    #[allow(dead_code)]
    pub fn random_in_unit_sphere() -> Vector3
    {
        loop
        {
            let random_point = Vector3::random(-1.0, 1.0);
            if random_point.length_squared() >= 1.0
            {
                continue;
            }
            return random_point;
        }
    }

    #[allow(dead_code)]
    pub fn to_color(&self) -> Color
    {
        return Color::new(self.x(), self.y(), self.z());//self.0.equal_approx(other.0);
    }

    #[allow(dead_code)]
    pub fn add_noise_on_angle(min: f64, max: f64) -> Vector3
    {
        // Find random angle between min & max inclusive
        let x_noise: f64 = (2.0 * PI * random_float(min, max) / 360.0).sin();
        let y_noise: f64 = (2.0 * PI * random_float(min, max) / 360.0).sin();
        let z_noise: f64 = (2.0 * PI * random_float(min, max) / 360.0).sin();
       
        // Convert Angle to Vector3
        let noise: Vector3 = Vector3::new(x_noise, y_noise, z_noise);
        return noise;
    }

    #[allow(dead_code)]
    pub fn random_unit_vector() -> Vector3
    {
        return Vector3::random_in_unit_sphere().normalize();
    }

    #[allow(dead_code)]
    pub fn random_in_hemisphere(normal: Vector3) -> Vector3
    {
        let in_unit_sphere: Vector3 = Vector3::random_in_unit_sphere();
        if Vector3::dot(in_unit_sphere, normal) > 0.0 // In the same hemisphere as the normal
        {
            return in_unit_sphere;
        }
        else
        {
            return Vector3::new(-in_unit_sphere.x(), -in_unit_sphere.y(), -in_unit_sphere.z());
        }
    }
}

impl From<Vector3> for Tuple
{
    #[allow(dead_code)]
    fn from(v: Vector3) -> Self
    {
        return v.0;
    }
}