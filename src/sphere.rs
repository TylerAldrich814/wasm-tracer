use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}
impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let halfb = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = halfb * halfb - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_disc = discriminant.sqrt();

        let mut root = (-halfb - sqrt_disc) / a;
        if root < t_min || t_max < root {
            root = (-halfb + sqrt_disc) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point - self.center) / self.radius;

        return true;
    }
}
