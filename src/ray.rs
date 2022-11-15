// use crate::constants::{degrees_to_radians, INFINITY, PI};
use crate::hit::{HitRecord, Hittable};
use crate::vec3::{cross, dot, unit_vector, Color, Point3, Vec3};
use core::f32::INFINITY;

pub struct Ray {
    origin: Point3,
    direction: Vec3<f32>,
}
impl Ray {
    pub fn new(origin: &Point3, direction: Vec3<f32>) -> Ray {
        Ray {
            origin: *origin,
            direction: direction,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3<f32> {
        &self.direction
    }
    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + t * self.direction
    }
}

pub fn ray_color(r: &Ray, world: &mut dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::white());
    }

    let unit_direction = unit_vector(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0);
}

pub fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let hb = dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = hb * hb - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-hb - discriminant.sqrt()) / a;
    };
}
