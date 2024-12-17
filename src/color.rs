use vec3;

type Color = vec3::Vec3;

fn write_color(file: &mut std::fs::File, color: Color) -> std::io::Result<()> {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;

    let pixel = format!("{} {} {}\n", ir, ig, ib);
    file.write(pixel.as_bytes())
}