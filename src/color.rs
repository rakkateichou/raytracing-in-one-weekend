use std::io::Write;

pub type Color = [f64; 3];

pub fn write_color(file: &mut std::fs::File, color: Color) -> std::io::Result<()> {
    let ir = (255.999 * color[0]) as i32;
    let ig = (255.999 * color[1]) as i32;
    let ib = (255.999 * color[2]) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    file.write_all(pixel.as_bytes())?;
    Ok(())
}