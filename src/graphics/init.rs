use crate::bsp::framebuffer::FrameBuffer;
use crate::bsp::mailbox::{send_property_messages, PropertyMessage};
use crate::info;

pub fn init_fb() -> FrameBuffer {
  let result = send_property_messages(&[
    PropertyMessage::SetPhysicalDimensions(640, 480),
    PropertyMessage::SetVirtualDimensions(640, 480),
    PropertyMessage::SetBitsPerPixel(24),
  ]);

  if let Ok(dimensions) = result {
    let buffer = send_property_messages(&[PropertyMessage::AllocateBuffer(16)]);

    if let Ok(buffer) = buffer {
      info!("Framebuffer located at {:#01x} size {:#01x}", buffer[5], buffer[6]);
      info!(
        "Working space located at {:#01x} size {:#01x}",
        buffer[5] + buffer[6],
        buffer[6]
      );

      return FrameBuffer::new(dimensions[5], dimensions[6], 24, buffer[5] as *mut u32, buffer[6]);
    }
  }

  panic!("Failed to initialize framebuffer");
}
