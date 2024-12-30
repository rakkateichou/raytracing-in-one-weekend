mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod util;
mod vec3;

use std::{fs::File, io::Write};

use camera::{Camera, CameraBuilder};
use hittable_list::HittableList;
use ray::{Point3, Ray};
use sphere::Sphere;
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::default();

    let sphere1 = Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    };
    let sphere2 = Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    };

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));

    let cam = CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .max_depth(50)
        .build();
    let render_result = cam.render(&world);

    let mut file = File::create("image.ppm")?;
    file.write_all(render_result.as_bytes())?;

    Ok(())
}
