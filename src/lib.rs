pub mod render;
pub mod vec3;
use crate::render::Render;

use core::sync::atomic::{AtomicU32, Ordering};

// todo: Add Settings for specific Canvas Sizes, and hook to JS
const WIDTH: usize = 1200; //800;
const HEIGHT: usize = 675; // 600;

pub type Buff = [u32; WIDTH * HEIGHT];

#[no_mangle]
pub static mut BUFFER: Buff = [0; WIDTH * HEIGHT];

// #[no_mangle]
static mut FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern "C" fn go() {
    Render::run(WIDTH, HEIGHT, &mut BUFFER);
}
