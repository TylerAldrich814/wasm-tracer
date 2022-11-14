// use crate::vec3::vec3_i32::{cross, dot, unit_vector, Color, Point3};
use crate::vec3::{cross, dot, unit_vector, Color, Point3, Vec3};
use crate::Buff;
use core::sync::atomic::{AtomicU32, Ordering};

pub struct Render {}
static mut FRAME: AtomicU32 = AtomicU32::new(0);

struct Ray {
    origin: Point3,
    direction: Vec3<f32>,
}
impl Ray {
    fn new(origin: &Point3, direction: Vec3<f32>) -> Ray {
        Ray {
            origin: *origin,
            direction: direction,
        }
    }
    fn origin(&self) -> &Point3 {
        &self.origin
    }
    fn direction(&self) -> &Vec3<f32> {
        &self.direction
    }
    fn at(&self, t: f32) -> Point3 {
        &self.origin + t * self.direction
    }
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

impl Render {
    pub unsafe fn run(width: usize, height: usize, buffer: &mut Buff) {
        let f = FRAME.fetch_add(1, Ordering::Relaxed);

        let aspect_ratio = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        for y in 0..height {
            for x in 0..width {
                // buffer[y * width + x] = f.wrapping_add((x ^ y) as u32 | 0xFF_00_00_00);
                let u = x as f32 / (width - 1) as f32;
                let v = y as f32 / (height - 1) as f32;

                let r = Ray::new(
                    &origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                let pixel_color = ray_color(&r);

                let pixel_colors: [u8; 4] = [
                    (pixel_color.x() * 255.999) as u8,
                    (pixel_color.y() * 255.999) as u8,
                    (pixel_color.z() * 255.999) as u8,
                    0x_FF,
                ];

                // buffer[y * width + x] = f.wrapping_add(as_u32_le(pixel_colors) | 0xFF_00_00_00);
                buffer[y * width + x] = as_u32_le(pixel_colors);
            }
        }
    }

    pub fn render(width: usize, height: usize, buffer: &mut Buff) {
        for y in 0..height {
            for x in 0..width {
                buffer[y * width + x] = (x ^ y) as u32 | 0xFF_00_00_00;
            }
        }
    }

    fn write_color(pixel_color: &Color) -> u32 {
        let rgbmax = 255.000;
        let r = (pixel_color.x() * rgbmax) as u8;
        let g = (pixel_color.y() * rgbmax) as u8;
        let b = (pixel_color.z() * rgbmax) as u8;

        return as_u32_le([r, g, b, 0xFF]);
    }
}

fn as_u32_le(arr: [u8; 4]) -> u32 {
    ((arr[0] as u32) << 0)
        + ((arr[1] as u32) << 8)
        + ((arr[2] as u32) << 16)
        + ((arr[3] as u32) << 24)
}

// pub fn examplerender_frame_safe(buffer: &mut Buff) {
//     unsafe {
//         // Not sure why we need a second unsafe here when this fn will be wrapped in unsafe..
//         let f = FRAME.fetch_add(1, Ordering::Relaxed);

//         for y in 0..HEIGHT {
//             for x in 0..WIDTH {
//                 buffer[y * WIDTH + x] = f.wrapping_add((x ^ y) as u32 | 0xFF_00_00_00);
//             }
//         }
//     }
// }
