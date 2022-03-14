//! System console

/// Console interfaces.
pub mod interface {
  pub use core::fmt;

  pub trait Write {
    /// Write a Rust format string.
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
  }
}
