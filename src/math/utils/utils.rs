use rand::Rng;
use crate::PI;

#[allow(dead_code)]
#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64
{
    return degrees * (PI as f64) / 180.0;
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_float(min: f64, max: f64) -> f64
{
    return rand::thread_rng().gen_range(min..max);
}

#[allow(dead_code)]
#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64
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