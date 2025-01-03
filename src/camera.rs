use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::{Point3, Ray},
    util::random_f64,
    vec3::Vec3,
};

use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Point3,
    pub max_depth: i32,
    pub sky_color: Color,

    samples_per_pixel: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn set_samples_per_pixel(&mut self, spp: i32) {
        self.samples_per_pixel = spp;
        self.pixel_samples_scale = 1.0 / spp as f64;
    }

    pub fn render(&self, world: &dyn Hittable) -> RgbImage {
        let mut buffer: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let pixel_count = self.image_width * self.image_height;
        let pb = ProgressBar::new(pixel_count as u64);

        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let r = self.get_ray(x as i32, y as i32);
                color += self.ray_color(&r, self.max_depth, world);
            }
            let result_color = color * self.pixel_samples_scale;
            *pixel = Rgb(color::color_rgb(result_color));
            pb.inc(1);
        }

        //for j in 0..self.image_height {
        //    eprint!("\rScanlines remaining: {} ", self.image_height - j);
        //    for i in 0..self.image_width {
        //        let mut color = Color::new(0.0, 0.0, 0.0);
        //        for sample in 0..self.samples_per_pixel {
        //            let r = self.get_ray(i, j);
        //            color += Self::ray_color(&r, self.max_depth, world);
        //        }
        //        let mid_result = color * self.pixel_samples_scale;
        //        result.push_str(&color::color_string(mid_result));
        //    }
        //}

        eprint!("Done\n");
        buffer
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(&ray, &Interval::new(0.001, std::f64::INFINITY)) {
            Some(hit) => {
                if let Some((scattered, attenuation)) = hit.mat.scatter(ray, &hit) {
                    attenuation * self.ray_color(&scattered, depth - 1, world)
                } else {
                    Color::default()
                }
            }
            None => {
                let unit_direction = ray.direction.unit_vector();
                let a = 0.5 * (unit_direction.y + 1.0);
                let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + self.sky_color * a;
                color
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));

        // if i == 0 && j == 0 {
        //     println!("{:?}", pixel_sample);
        // }

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}

#[derive(Debug)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    center: Point3,
    max_depth: i32,
    samples_per_pixel: i32,
    sky_color: Color,
}

impl CameraBuilder {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let center = Vec3::new(0.0, 0.0, 0.0);
        let max_depth = 10;
        let samples_per_pixel = 10;
        let sky_color = Color::new(0.5, 0.7, 1.0);

        Self {
            aspect_ratio,
            image_width,
            center,
            max_depth,
            samples_per_pixel,
            sky_color,
        }
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn center(mut self, center: Point3) -> Self {
        self.center = center;
        self
    }

    pub fn max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn sky_color(mut self, sky_color: Color) -> Self {
        self.sky_color = sky_color;
        self
    }

    pub fn build(self) -> Camera {
        let CameraBuilder {
            aspect_ratio,
            image_width,
            center,
            max_depth,
            samples_per_pixel,
            sky_color,
        } = self;

        let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            max_depth,
            sky_color,
            samples_per_pixel,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }
}
