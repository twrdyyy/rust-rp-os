#[cfg(any(feature = "bsp_rpi3"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi3"))]
pub use raspberrypi::*;