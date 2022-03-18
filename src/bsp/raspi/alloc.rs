use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use bare_metal::Mutex;
use linked_list_allocator::Heap;

use crate::cpu::free;

pub struct SharedHeap(Mutex<RefCell<Heap>>);

impl SharedHeap {
  pub const fn empty() -> SharedHeap {
    SharedHeap(Mutex::new(RefCell::new(Heap::empty())))
  }

  pub fn init(&self, start_addr: usize, size: usize) {
    free(|cs| unsafe {
      if let Ok(mut lock) = self.0.borrow(*cs).try_borrow_mut() {
        lock.init(start_addr, size);
      } else {
        panic!("Failed to allocate heap!");
      }
    });
  }
}

unsafe impl GlobalAlloc for SharedHeap {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    free(|cs| {
      if let Ok(mut lock) = self.0.borrow(*cs).try_borrow_mut() {
        lock
        .allocate_first_fit(layout)
        .ok()
        .map_or(ptr::null_mut(), |a| a.as_ptr())
      } else {
        panic!("Failed to get lock on shared heap. Already borrowed?");
      } 
    })
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    free(|cs| {
      self
        .0
        .borrow_ref_mut(*cs)
        .deallocate(NonNull::new_unchecked(ptr), layout)
    })
  }
}

#[alloc_error_handler]
fn on_oom(layout: Layout) -> ! {
  panic!("Failed to allocate: {:?}", layout)
}

#[global_allocator]
pub static ALLOCATOR: SharedHeap = SharedHeap::empty();
