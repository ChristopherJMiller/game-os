#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use core::cell::UnsafeCell;

use bsp::alloc::ALLOCATOR;

use crate::bsp::{mailbox::{PropertyMessage, send_property_messages}, framebuffer::FrameBuffer};

mod cpu;
mod bsp;
mod panic_wait;
mod io;
mod time;

extern "Rust" {
  static __bss_end_exclusive: UnsafeCell<()>;
}

// Memory Locations

fn heap_start() -> usize {
  unsafe { __bss_end_exclusive.get() as usize }
}

const fn heap_size() -> usize {
  1024 * 1024 * 128  // 128 MiB
}

fn heap_end() -> usize {
  heap_start() + heap_size() + 1
}

fn mailbox_heap_location() -> usize {
  heap_end() + (16 - (heap_end() % 16))
}

fn init_heap() {
  let heap_start = heap_start();
  let heap_size = heap_size();
  info!("Allocating Heap {:#01x}-{:#01x}", heap_start, heap_start + heap_size);
  ALLOCATOR.init(heap_start, heap_size);
  info!("Mailbox Heap Location {:#01x}", mailbox_heap_location())
}

unsafe fn kernel_main() -> ! {
  // Init Heap
  init_heap();

  info!("Hello from Rust!");
  
  let result = send_property_messages(&[
    PropertyMessage::SetPhysicalDimensions(640, 480),
    PropertyMessage::SetVirtualDimensions(640, 480),
    PropertyMessage::SetBitsPerPixel(24)
  ]);

  if let Ok(dimensions) = result {
    let buffer = send_property_messages(&[
      PropertyMessage::AllocateBuffer(16)
    ]);

    if let Ok(buffer) = buffer {
      let framebuffer = FrameBuffer::new(
        dimensions[5], 
        dimensions[6],
        24,
        buffer[5] as *mut u32
      );
      info!("Width {}", dimensions[5]);
      info!("Height {}", dimensions[6]);
      info!("Buffer Location {:#01x}", buffer[5]);
      info!("Buffer Size {:#01x}", buffer[6]);

      for x in 5..100 {
        for y in 5..100 {
          framebuffer.draw_pixel([x, y], &[0xff, 0xff, 0xff]);
        }
      }

    }
  }


  info!("After it!");

  panic!("Stopping here.")
}
