
use core::cell::UnsafeCell;

use crate::{bsp::alloc::ALLOCATOR, info};

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

pub fn mailbox_heap_location() -> usize {
  heap_end() + (16 - (heap_end() % 16))
}

pub fn init_heap() {
  let heap_start = heap_start();
  let heap_size = heap_size();
  info!("Allocating Heap {:#01x}-{:#01x}", heap_start, heap_start + heap_size);
  ALLOCATOR.init(heap_start, heap_size);
  info!("Mailbox Heap Location {:#01x}", mailbox_heap_location())
}
