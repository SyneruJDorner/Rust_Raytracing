use crate::Tuple;

mod operators
{
    pub mod default;
    pub mod mul;
    pub mod add;
    pub mod sub;
    pub mod div;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(Tuple);

impl Color
{
    #[allow(dead_code)]
    pub fn new(r: f64, g: f64, b: f64) -> Color
    {
        return Color(Tuple::new(r, g, b, 0.0));
    }

    #[allow(dead_code)]
    pub fn black() -> Color
    {
        return Color(Tuple::new(0.0, 0.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn white() -> Color
    {
        return Color(Tuple::new(1.0, 1.0, 1.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn red() -> Color
    {
        return Color(Tuple::new(1.0, 0.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn lime() -> Color
    {
        return Color(Tuple::new(0.0, 1.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn blue() -> Color
    {
        return Color(Tuple::new(0.0, 0.0, 1.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn yellow() -> Color
    {
        return Color(Tuple::new(1.0, 1.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn cyan() -> Color
    {
        return Color(Tuple::new(0.0, 1.0, 1.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn magenta() -> Color
    {
        return Color(Tuple::new(1.0, 0.0, 1.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn silver() -> Color
    {
        return Color(Tuple::new(0.75294117647, 0.75294117647, 0.75294117647, 0.0));
    }

    #[allow(dead_code)]
    pub fn gray() -> Color
    {
        return Color(Tuple::new(0.50196078431, 0.50196078431, 0.50196078431, 0.0));
    }

    #[allow(dead_code)]
    pub fn maroon() -> Color
    {
        return Color(Tuple::new(0.50196078431, 0.0, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn olive() -> Color
    {
        return Color(Tuple::new(0.50196078431, 0.50196078431, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn green() -> Color
    {
        return Color(Tuple::new(0.0, 0.50196078431, 0.0, 0.0));
    }

    #[allow(dead_code)]
    pub fn purple() -> Color
    {
        return Color(Tuple::new(0.50196078431, 0.0, 0.50196078431, 0.0));
    }

    #[allow(dead_code)]
    pub fn pink() -> Color
    {
        return Color(Tuple::new(0.98431372549, 0.28235294117, 0.76862745098, 0.0));
    }

    #[allow(dead_code)]
    pub fn teal() -> Color
    {
        return Color(Tuple::new(0.0, 0.50196078431, 0.50196078431, 0.0));
    }

    #[allow(dead_code)]
    pub fn navy() -> Color
    {
        return Color(Tuple::new(0.0, 0.0, 0.50196078431, 0.0));
    }

    #[allow(dead_code)]
    pub fn r(&self) -> f64
    {
        return self.0.x;
    }

    #[allow(dead_code)]
    pub fn g(&self) -> f64
    {
        return self.0.y;
    }

    #[allow(dead_code)]
    pub fn b(&self) -> f64
    {
        return self.0.z;
    }

    #[allow(dead_code)]
    pub fn rgb(&self) -> Color
    {
        return Color::new(self.r(), self.g(), self.b()); 
    }

    #[allow(dead_code)]
    pub fn r_u8(&self) -> u8
    {
        return (self.r() * 255.0) as u8;
    }

    #[allow(dead_code)]
    pub fn g_u8(&self) -> u8
    {
        return (self.g() * 255.0) as u8;
    }

    #[allow(dead_code)]
    pub fn b_u8(&self) -> u8
    {
        return (self.b() * 255.0) as u8;
    }

    #[allow(dead_code)]
    pub fn equal_approx(&self, other: Color) -> bool
    {
        return self.0.equal_approx(other.0);
    }

    #[allow(dead_code)]
    pub fn store_color(rgb: Color, samples_per_pixel: u32) -> [f64; 3]
    {
        let mut r = rgb.r();
        let mut g = rgb.g();
        let mut b = rgb.b();

        let scale = 1.0 / (samples_per_pixel as f64);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        // let ir = (255.0 * clamp(r, 0.0, 1.0)) as i32;
        // let ig = (255.0 * clamp(g, 0.0, 1.0)) as i32;
        // let ib = (255.0 * clamp(b, 0.0, 1.0)) as i32;

        return [r, g, b]//format!("{} {} {}", r, g, b);
    }

    // pub fn from_string(s: &str) -> Color
	// {
    //     let values: Vec<&str> = s.split_whitespace().collect();
    //     let r = values[0].parse::<f64>();//.ok()? as f64 / 255.0;
    //     let g = values[1].parse::<f64>();//.ok()? as f64 / 255.0;
    //     let b = values[2].parse::<f64>();//.ok()? as f64 / 255.0;
    //     return Color::new(r, g, b)
    // }

    pub fn clamp(&self) -> Color {
        let clamp_val = |x: f64|
        {
            if x < 0.0 {
                0.0
            } else if x > 1.0 {
                1.0
            } else {
                x
            }
        };
        Color::new(clamp_val(self.r()), clamp_val(self.g()), clamp_val(self.b()))
    }
}
