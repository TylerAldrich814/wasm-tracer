//pub static INFINITY: f32 = std::f32::INFINITY;
pub static INFINITY: f32 = std::f32::MAX;
pub static PI: f32 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * PI / 180.0
}
