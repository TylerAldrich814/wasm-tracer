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

// NOTE: Webassembly is little-endian, so when we inscribe
//       our RGB  pixel data into a 4BYte u32. We need to
//       make sure its reversed!
//       i.e
//       Big Endian = u32 :: R_G_B_A [ red, green, blue, alpha ]
//       Lil Endian = u32 :: A_B_G_A [ red, green, blue, alpha ]
//       so, we'll encode it as 0xFF_FF_FF_FF
//                            [ 0xAlpha_Blue_Green_Red ]
pub fn render_frame_safe(buffer: &mut Buff) {
    unsafe {
        // Not sure why we need a second unsafe here when this fn will be wrapped in unsafe..
        let f = FRAME.fetch_add(1, Ordering::Relaxed);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[y * WIDTH + x] = f.wrapping_add((x ^ y) as u32 | 0xFF_00_00_00);
            }
        }
    }
}

pub fn render_frame_trig(buffer: &mut Buff) {
    unsafe {
        let f = FRAME.fetch_add(1, Ordering::Relaxed);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let v = (x as f32).sin() * 255.0 + (y as f32).sin() * 255.0;
                buffer[y * WIDTH + x] = f.wrapping_add(v as u32) | 0xFF_00_00_00;
            }
        }
    }
}
