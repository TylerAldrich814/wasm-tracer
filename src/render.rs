// use crate::vec3::vec3_i32::{cross, dot, unit_vector, Color, Point3};
use crate::constants::{degrees_to_radians, INFINITY, PI};
use crate::hit_lists::HitList;
use crate::ray::{ray_color, Ray};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use crate::Buff;
//use core::sync::atomic::{AtomicU32, Ordering};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Render {}
//static mut FRAME: AtomicU32 = AtomicU32::new(0);

impl Render {
    pub unsafe fn run(width: usize, height: usize, buffer: &mut Buff) {
        // let f = FRAME.fetch_add(1, Ordering::Relaxed);

        // Image
        let aspect_ratio = 16.0 / 9.0;

        // World
        let mut world = HitList::new();
        let sphere_one = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let sphere_two = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.);
        world.push(Box::new(sphere_one));
        world.push(Box::new(sphere_two));

        // Camera
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

        let mut ray_y: usize = height; // needed to flip image to correct orientation
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / (width - 1) as f32;
                // let v = y as f32 / (height - 1) as f32;
                let v = ray_y as f32 / (height - 1) as f32;

                let r = Ray::new(
                    &origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                let pixel_color = ray_color(&r, &mut world);

                let pixel_colors: [u8; 4] = [
                    (pixel_color.x() * 255.999) as u8,
                    (pixel_color.y() * 255.999) as u8,
                    (pixel_color.z() * 255.999) as u8,
                    0x_FF,
                ];

                buffer[y * width + x] = as_u32_le(pixel_colors);
            }
            ray_y -= 1;
        }
    }

    pub fn render(width: usize, height: usize, buffer: &mut Buff) {
        for y in 0..height {
            for x in 0..width {
                buffer[y * width + x] = (x ^ y) as u32 | 0xFF_00_00_00;
            }
        }
    }
}

// Takes an Array of 4 u8's and converts them into an u32, little Endian
fn as_u32_le(arr: [u8; 4]) -> u32 {
    ((arr[0] as u32) << 0)
        + ((arr[1] as u32) << 8)
        + ((arr[2] as u32) << 16)
        + ((arr[3] as u32) << 24)
}
