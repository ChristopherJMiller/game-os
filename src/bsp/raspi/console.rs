// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

//! Console BSP

use bare_metal::Mutex;

use crate::{io::console, cpu::free};
use core::{fmt::{self, Arguments, Result}, cell::RefCell};

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

struct QEMUOutputInner;

struct QEMUOutput {
  inner: Mutex<RefCell<QEMUOutputInner>>,
}

impl QEMUOutput {
  pub const fn new() -> Self {
    Self { 
      inner: Mutex::new(RefCell::new(QEMUOutputInner::new()))
    }
  }
}

impl QEMUOutputInner {
  pub const fn new() -> Self {
    QEMUOutputInner {}
  }

  fn write_char(&mut self, c: char) {
    unsafe {
      core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
    }
  }
}

impl fmt::Write for QEMUOutputInner {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      match c {
        '\n' => self.write_char('\n'),
        x => self.write_char(x),
      }
    }

    Ok(())
  }
}

impl console::interface::Write for QEMUOutput {
  fn write_fmt(&self, args: Arguments) -> Result {
    free(|cs| {
      if let Err(err) = fmt::Write::write_fmt(&mut *self.inner.borrow_ref_mut(*cs), args) {
        panic!("{}", err);
      }
    });

    Ok(())
  }
}

/// Return a reference to the console.
pub fn console() -> &'static impl console::interface::Write {
  &QEMU_OUTPUT
}
