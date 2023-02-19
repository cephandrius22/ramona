#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::{fs::File, io::Error};
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;

mod material;
use material::{Dialetric, Lambertian, Metal};

// I'm not sure that I'm doing this correctly.
mod util;
use util::{Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3};

mod camera;
use camera::Camera;

use rand::{rngs::ThreadRng, Rng};

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}

fn calculate_color(color: Vec3, samples_per_pixel: i32) -> Vec3 {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f32;

    // sqrt is for gamma correction
    r = f32::sqrt(r * scale);
    g = f32::sqrt(g * scale);
    b = f32::sqrt(b * scale);

    Vec3 {
        x: clamp(r, 0.0, 0.999) * 256.0,
        y: clamp(g, 0.0, 0.999) * 256.0,
        z: clamp(b, 0.0, 0.999) * 256.0,
    }
}

/// Determine the color of a pixel for a given ray.
fn color_pixel(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(*ray, 0.01, 99999999999.0) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
            // Don't overload the * operator to do dot product...
            let res = color_pixel(&scattered, world, depth - 1);
            let vec = Vec3 {
                x: res.x * attenuation.x,
                y: res.y * attenuation.y,
                z: res.z * attenuation.z,
            };
            return vec;
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    // white-ish background
    Vec3::new(0.8, 0.8, 0.8)
}

fn generate_large_scene(rng: &mut ThreadRng, world: &mut HittableList) {
    let material_ground = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(material_ground),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f32 + (0.9 * rng.gen_range(0.0..1.0)),
                0.2,
                b as f32 + (0.9 * rng.gen_range(0.0..1.0)),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) - Color::random(0.0, 1.0);
                    let sphere_material = Lambertian { albedo };
                    world.add(Sphere::new(center, 0.2, Rc::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.add(Sphere::new(center, 0.2, Rc::new(sphere_material)));
                } else {
                    let sphere_material = Dialetric {
                        index_of_refraction: 1.5,
                    };
                    world.add(Sphere::new(center, 0.2, Rc::new(sphere_material)));
                }
            }
        }

        let material1 = Dialetric {
            index_of_refraction: 1.5,
        };
        world.add(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(material1),
        ));

        let material2 = Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        };
        world.add(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(material2),
        ));

        let material3 = Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        };
        world.add(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(material3),
        ));
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let samples_per_pixel = 50;

    let mut rng = rand::thread_rng();
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const WIDTH: u32 = 900;
    const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        10.0,
    );

    let mut world = HittableList::new();

    // generate_large_scene(&mut rng, &mut world);

    let material_ground = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(material_ground),
    ));

    for _a in -1..1 {
        let material1 = Dialetric {
            index_of_refraction: 1.5,
        };
        world.add(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(material1),
        ));

        let material2 = Lambertian {
            albedo: Color::new(0.4, 0.2, 0.8),
        };
        world.add(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(material2),
        ));

        let material3 = Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        };
        world.add(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(material3),
        ));
    }

    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);

    let mut image: Vec<u8> = Vec::new();

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                // u and v are the how far, as a percentage, x and y are from
                // the vertical and horizontal of our viewport. This is used
                // to map our pixel coords to the "camera" coords.
                let u = (i as f32 + rng.gen::<f32>()) / (WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (HEIGHT - 1) as f32;

                // origin is the camera (0, 0 ,0) and direction is the point in
                // the viewport whose color value we are calculating.
                let ray = camera.get_ray(u, v);
                pixel_color += color_pixel(&ray, &world, 50);
            }

            let color = calculate_color(pixel_color, samples_per_pixel);
            let ir = (color.x) as u8;
            let ig = (color.y) as u8;
            let ib = (color.z) as u8;
            image.push(ir);
            image.push(ig);
            image.push(ib);
            image.push(255);
        }
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image).unwrap();

    Ok(())
}
