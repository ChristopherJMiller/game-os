use embedded_graphics::{draw_target::DrawTarget, prelude::{OriginDimensions, Size, RgbColor}, pixelcolor::Rgb888, Pixel};

use crate::warn;

pub struct FrameBuffer {
  pub bytes_per_pixel: usize,
  pub width: u32,
  pub height: u32,
  pub pitch: u32,
  pub working_buf: *mut u32,
  pub buf: *mut u32,
  pub buf_size: u32,
}

impl FrameBuffer {
  pub fn new(width: u32, height: u32, depth: u32, buf: *mut u32, buf_size: u32) -> Self {
    let working_buf = (buf as u32 + buf_size) as *mut u32;
    unsafe { core::ptr::write_bytes(working_buf, 0, buf_size as usize) }
    Self {
      bytes_per_pixel: (depth / 8) as usize,
      width,
      height,
      pitch: width * (depth / 8),
      buf,
      buf_size,
      working_buf
    }
  }

  pub fn draw_pixel(&self, [x, y]: [u32; 2], pixel: &[u8; 3]) {
    let offset = (y * self.pitch) + (x * self.bytes_per_pixel as u32);
    // Check bounds
    if (self.width - 1) < x || (self.height - 1) < y {
      warn!("Attempted to write outside of frame!");
      return;
    }
    let location: *mut u8 = (self.working_buf as u32 + offset) as *mut u8;
    unsafe { core::ptr::copy(pixel.as_ptr(), location, self.bytes_per_pixel) };
  }

  pub fn update_fb(&self) {
    unsafe { core::ptr::copy(self.working_buf, self.buf, self.buf_size as usize) }
  }
}

impl DrawTarget for FrameBuffer {
  type Color = Rgb888;
  type Error = core::convert::Infallible;

  fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>> {
    for Pixel(coord, color) in pixels.into_iter() {
      let (x, y) = coord.into();
      let (x, y) = (x.min(self.width as i32), y.min(self.height as i32));
        self.draw_pixel([x as u32, y as u32], &[color.r(), color.g(), color.b()]);
    }

    Ok(())
  }
}

impl OriginDimensions for FrameBuffer {
  fn size(&self) -> Size {
    Size::new(self.width, self.height)
  }
}
