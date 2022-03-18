use alloc::boxed::Box;


pub struct FrameBuffer {
  pub bytes_per_pixel: usize,
  pub width: u32,
  pub height: u32,
  pub pitch: u32,
  pub buf: *mut u32
}

impl FrameBuffer {
  pub fn new(width: u32, height: u32, depth: u32, buf: *mut u32) -> Self {
    Self {
      bytes_per_pixel: (depth / 8) as usize,
      width,
      height,
      pitch: width * (depth / 8),
      buf,
    }
  }

  pub fn draw_pixel(&self, [x, y]: [u32; 2], pixel: &[u8; 3]) {
    let location: *mut u8 = (self.buf as u32 + (y * self.pitch) + (x * self.bytes_per_pixel as u32)) as *mut u8;
    unsafe { core::ptr::copy(pixel.as_ptr(), location, self.bytes_per_pixel) };
  }
}
