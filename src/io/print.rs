//! Console Printing

use crate::bsp;
use super::console;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use console::interface::Write;

  bsp::console::console().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _panic_print(args: fmt::Arguments) {
  use console::interface::Write;

  bsp::console::new_console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints without a newline. Used with panicking, as it creates new object instances to bypass locks.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! panic_print {
  ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}


/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
  () => ($crate::panic_print!("\n"));
  ($($arg:tt)*) => ({
    $crate::io::_panic_print(format_args_nl!($($arg)*));
  })
}

/// Prints with a newline. Used with panicking, as it creates new object instances to bypass locks.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! panic_println {
  () => ($crate::panic_print!("\n"));
  ($($arg:tt)*) => ({
    $crate::io::_panic_print(format_args_nl!($($arg)*));
  })
}

/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
  ($string:expr) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::io::print::_print(format_args_nl!(
      concat!("[I {:>3}.{:03}{:03}] ", $string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000
    ));
  });
  ($format_string:expr, $($arg:tt)*) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::io::print::_print(format_args_nl!(
      concat!("[I {:>3}.{:03}{:03}] ", $format_string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000,
      $($arg)*
    ));
  })
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
  ($string:expr) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::io::print::_print(format_args_nl!(
      concat!("[W {:>3}.{:03}{:03}] ", $string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000
    ));
  });
  ($format_string:expr, $($arg:tt)*) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::io::print::_print(format_args_nl!(
      concat!("[W {:>3}.{:03}{:03}] ", $format_string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000,
      $($arg)*
    ));
  })
}
