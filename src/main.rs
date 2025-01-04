mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::CameraBuilder;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use ray::{Point3, Ray};
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut world = HittableList::default();

    let material_ground = Box::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Box::new(Dielectric {
        refraction_index: 1.50
    });
    let material_bubble = Box::new(Dielectric {
        refraction_index: 1.0 / 1.50
    });
    let material_right = Box::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let sphere_ground = Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat: material_ground,
    };
    let sphere_left = Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_left,
    };
    let sphere_bubble = Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.4,
        mat: material_bubble,
    };
    let sphere_center = Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.2,
        },
        radius: 0.5,
        mat: material_center,
    };
    let sphere_right = Sphere {
        center: Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat: material_right,
    };

    world.add(sphere_ground);
    world.add(sphere_left);
    world.add(sphere_bubble);
    world.add(sphere_center);
    world.add(sphere_right);

    let cam = CameraBuilder::new()
        // .sky_color(Color::new(1.0, 0.4, 0.5))
        .aspect_ratio(16.0 / 9.0)
        .image_width(600)
        .max_depth(100)
        .build();

    cam.render(&world)
        .save("output.png")
        .expect("Failed to save image");
}
