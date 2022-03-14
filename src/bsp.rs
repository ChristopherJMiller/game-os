#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspi;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspi::*;
