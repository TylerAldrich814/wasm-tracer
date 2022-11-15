use std::cell::RefCell;
use std::rc::Rc;

use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HitList {
    objects: Vec<Rc<RefCell<Box<dyn Hittable>>>>,
}

impl HitList {
    pub fn new() -> HitList {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(Rc::new(RefCell::new(obj)));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn with_capacity(size: usize) -> HitList {
        Self {
            objects: Vec::with_capacity(size),
        }
    }
}

impl Hittable for HitList {
    fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|obj| {
            if obj
                .borrow_mut()
                .hit(ray, t_min, closest_so_far, &mut temp_record)
            {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record;
            }
        });

        return hit_anything;
    }
}
