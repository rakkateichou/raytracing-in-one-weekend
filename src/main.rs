mod vec3;
mod color;
mod ray;

use std::fs::File;
use std::io::Write;

use vec3::Vec3;
use ray::Ray;
use color::Color;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = ray.direction.dot(&oc) * -2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return [1.0, 0.0, 0.0].into();
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
    color
}

fn main() -> std::io::Result<()> {

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width= 400;

    // Calculate the image height

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Deltas

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Locations of the upper left pixel
    let viewport_upper_left = camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render

    let mut file = File::create("image.ppm")?;
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write_all(header.as_bytes())?;

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            color::write_color(&mut file, pixel_color)?;
        }
    }

    eprint!("\rDone.                      \n");
    Ok(())
}
