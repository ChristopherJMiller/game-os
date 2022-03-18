use alloc::boxed::Box;
use embedded_graphics::{draw_target::DrawTarget, prelude::{PixelColor, OriginDimensions, Size, RgbColor}, pixelcolor::Rgb888, Pixel};


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
