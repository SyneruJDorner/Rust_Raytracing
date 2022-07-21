
use crate::Color;
use crate::Ray;
use crate::HitRecord;
use crate::Scatterable;
use crate::Emmitable;
use crate::Normalable;

#[derive(Debug, Clone, Copy)]
pub struct Emmision
{
    pub emit: Color
}

impl Emmision
{
    #[allow(dead_code)]
    pub fn new(r: f64, g: f64, b: f64) -> Emmision
    {
        Emmision
        {
            emit: Color::new(r, g, b)
        }
    }

    #[allow(dead_code)]
    pub fn from_color(color: Color) -> Emmision
    {
        Emmision
        {
            emit: color
        }
    }
}

impl Scatterable for Emmision
{
    #[allow(unused_variables)]
    fn scatter(&self, hit_record: HitRecord) -> Option<(Ray, Color)>
    {
        return None;
    }
}

impl Emmitable for Emmision
{
    #[allow(unused_variables)]
    fn emitted(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(self.emit);
    }
}

impl Normalable for Emmision
{
    #[allow(unused_variables)]
    fn normals(&self, hit_record: HitRecord) -> Option<Color>
    {
        return Some(hit_record.get_normal().to_color());
    }
}