#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

#[cfg(target_arch = "aarch64")]
pub use arch_cpu::*;
