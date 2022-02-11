use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

struct BufferWrapper(Vec<u32>);

impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
    }
}

impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u8, self.0.len() * 4) }
    }
}

impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u32, self.0.len()) }
    }
}

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper(vec![0u32; WIDTH * HEIGHT]);

    let mut window = Window::new("sample", WIDTH, HEIGHT, WindowOptions::default())?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.5));
        fill_random(&mut buf);
        window.update_with_buffer(buf.borrow())?;
    }

    Ok(())
}

fn fill_random(buf: &mut BufferWrapper) {
    let mut rng = rand::thread_rng();
    let sample: f64 = rng.gen();
    for (i, x) in buf.0.iter_mut().enumerate() {
        *x = (i as f64 * sample) as u32;
    }
}
