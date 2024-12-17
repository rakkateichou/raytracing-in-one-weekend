mod color;

use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    let mut file = File::create("image.ppm")?;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(header.as_bytes());

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color = color::Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            color::write_color(&mut file, pixel_color);
        }
    }

    eprint!("\rDone.                      \n");
    Ok(())
}