use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn color_rgb(color: Color) -> [u8; 3]
{
    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(color[0])) as u8;
    let ig = (256.0 * intensity.clamp(color[1])) as u8;
    let ib = (256.0 * intensity.clamp(color[2])) as u8;

    [ir, ig, ib]
}
