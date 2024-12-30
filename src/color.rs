use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn color_string(color: Color) -> String
{
    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(color[0])) as i32;
    let ig = (256.0 * intensity.clamp(color[1])) as i32;
    let ib = (256.0 * intensity.clamp(color[2])) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    pixel
}
