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
    if hit_sphere(&Point3::new(0., 0.1, -1.0), 0.5, r) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = unit_vector(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(&oc, r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    return discriminant > 0.0;
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
}

// Takes an Array of 4 u8's and converts them into an u32, little Endian
fn as_u32_le(arr: [u8; 4]) -> u32 {
    ((arr[0] as u32) << 0)
        + ((arr[1] as u32) << 8)
        + ((arr[2] as u32) << 16)
        + ((arr[3] as u32) << 24)
}
