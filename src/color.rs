use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(x: f64) -> f64
{
    if x > 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}

pub fn color_rgb(color: Color) -> [u8; 3]
{
    
    let r = linear_to_gamma(color[0]);
    let g = linear_to_gamma(color[1]);
    let b = linear_to_gamma(color[2]);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as u8;
    let ig = (256.0 * intensity.clamp(g)) as u8;
    let ib = (256.0 * intensity.clamp(b)) as u8;

    [ir, ig, ib]
}
