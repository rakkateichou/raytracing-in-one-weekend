use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

fn linear_to_gamma(color: Color) -> Color
{
    let mut result = color.clone();
    if color[0] > 0.0 {
        result[0] = color[0].sqrt();
    }
    if color[1] > 0.0 {
        result[1] = color[1].sqrt();
    }
    if color[2] > 0.0 {
        result[2] = color[2].sqrt();
    }
    result
}

pub fn color_rgb(color: Color) -> [u8; 3]
{
    let Color {
        x: r, y: g, z: b
    } = linear_to_gamma(color);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as u8;
    let ig = (256.0 * intensity.clamp(g)) as u8;
    let ib = (256.0 * intensity.clamp(b)) as u8;

    [ir, ig, ib]
}
