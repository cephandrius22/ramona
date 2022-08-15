#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// I'm not sure that I'm doing this correctly.
mod util;
use util::{HitRecord, Hittable, HittableList, Point3, Ray, Sphere, Vec3};

mod camera;
use camera::Camera;

use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}

fn write_color(color: Vec3, samples_per_pixel: i32) -> Vec3 {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    Vec3 {
        x: clamp(r, 0.0, 0.999) * 256.0,
        y: clamp(g, 0.0, 0.999) * 256.0,
        z: clamp(b, 0.0, 0.999) * 256.0,
    }
}

/// Determine the color of a pixel for a given ray.
fn color_pixel(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(*ray, 0.0, 99999999999.0, &mut rec) {
        return Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    // gradient background
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let samples_per_pixel = 10;

    // let aspect_ratio: f32 = WIDTH as f32 / HEIGHT as f32;
    // let focal_length: f32 = 1.0; // distance to camera

    // let viewport_height = 2.0;
    // let viewport_width = aspect_ratio * viewport_height;

    // let origin = Point3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner =
    //     origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    let camera = Camera::new();

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let mut rng = rand::thread_rng();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // world.draw(pixels.get_frame());
            let frame = pixels.get_frame();
            println!("Drawing");
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                // x and y are pixel coordinates
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;

                // image was displaying upside down for some reason.
                let y = HEIGHT as i16 - y;

                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for s in 0..samples_per_pixel {
                    let u = (x as f32 + rng.gen::<f32>()) as f32 / (WIDTH - 1) as f32;
                    let v = (y as f32 + rng.gen::<f32>()) as f32 / (HEIGHT - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    pixel_color += color_pixel(&ray, &world);
                }
                // let u = x as f32 / (WIDTH - 1) as f32;
                // let v = y as f32 / (HEIGHT - 1) as f32;
                // let ray = camera.get_ray(u, v);
                // pixel_color += color_pixel(&ray, &world);

                // u and v are the how far, as a percentage, x and y are from
                // the vertical and horizontal of our viewport. This is used
                // to map our pixel coords to the "camera" coords.

                // origin is the camera (0, 0 ,0) and direction is the point in
                // the viewport whose color value we are calculating.
                // let ray = Ray {
                //     origin: origin,
                //     direction: lower_left_corner + horizontal * u + vertical * v - origin,
                // };

                let color = write_color(pixel_color, samples_per_pixel);
                let ir = (color.x) as u8;
                let ig = (color.y) as u8;
                let ib = (color.z) as u8;

                let rgba = [ir, ig, ib, 0xff];

                pixel.copy_from_slice(&rgba);
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    });
}
