// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

//! Console BSP

use crate::io::console;
use core::fmt;

#[derive(Default)]
struct QEMUOutput;

impl fmt::Write for QEMUOutput {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      unsafe {
        core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
      }
    }

    Ok(())
  }
}

/// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
  QEMUOutput::default()
}
