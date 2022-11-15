// use std::cell::RefCell;
// use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3<f32>) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}
pub trait Hittable {
    fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
