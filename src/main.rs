#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod util;
use util::{Vec3,Point3, Ray}; // this makes no sense to me...maybe it does now

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
}

// struct Camera {
//     origin: Point3,
//     viewport_height: f32,
//     viewport_width: f32,
//     lower_left_corner: Point3,
// }

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}


fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction * ray.direction;
    let half_b = oc * ray.direction;
    let c = (oc * oc) - (radius * radius);
    let discriminant: f32 = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f32::sqrt(discriminant)) / a;
    }
}

fn color_pixel(ray: &Ray) -> Vec3 {
    let mut t = hit_sphere(Point3 {x: 0.0, y: 0.0, z: -1.0}, 0.5, ray);
    if t > 0.0 {
        let normal = unit_vector(ray.at(t) - Vec3{x: 0.0, y: 0.0, z: -1.0});
        return Vec3{x: normal.x + 1.0, y: normal.y + 1.0, z: normal.z + 1.0} * 0.5;
    }
    let unit_direction = unit_vector(ray.direction);
    t = 0.5 * (unit_direction.y + 1.0);
    Vec3{x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + Vec3{x: 0.5, y: 0.7, z: 1.0} * (t)
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
    let mut world = World::new();

    let aspect_ratio: f32 = WIDTH as f32 / HEIGHT as f32;
    let focal_length: f32 = 1.0; // distance to camera

    
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let origin = Point3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3 {x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0)  - Vec3 {x: 0.0, y: 0.0, z: focal_length};

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // world.draw(pixels.get_frame());
            let frame = pixels.get_frame();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;
                let y = HEIGHT as i16 - y; // image was displaying upside down for some reason.
                
                let u = x as f32 / (WIDTH - 1) as f32;
                let v = y as f32 / (HEIGHT - 1) as f32;
                let ray = Ray {origin: origin, direction: lower_left_corner + horizontal * u + vertical * v - origin};

                let color = color_pixel(&ray);
                let ir = (255.9999 * color.x) as u8;
                let ig = (255.9999 * color.y) as u8;
                let ib = (255.9999 * color.z) as u8;
    
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

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {}
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     let x = (i % WIDTH as usize) as i16;
        //     let y = (i / WIDTH as usize) as i16;

        //     let u = x as f32 / (WIDTH - 1) as f32;
        //     let v = y as f32 / (HEIGHT - 1) as f32;
        //     let ray = Ray {origin: origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};

        //     let rgba = [1, 1, 1, 0xff];

        //     pixel.copy_from_slice(&rgba);
        // }
    }
}
