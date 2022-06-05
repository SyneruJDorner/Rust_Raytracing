use rand::Rng;
use crate::constants::PI;

#[allow(dead_code)]
#[inline(always)]
pub fn degrees_to_radians(degrees: f32) -> f32
{
    return degrees * (PI as f32) / 180.0;
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_float(min: f32, max: f32) -> f32
{
    return rand::thread_rng().gen_range(min..max);
}

#[allow(dead_code)]
#[inline(always)]
pub fn clamp(x: f32, min: f32, max: f32) -> f32
{
    if x < min
    {
        return min;
    }

    if x > max
    {
        return max;
    }

    return x;
}