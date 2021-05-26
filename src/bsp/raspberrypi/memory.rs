use core::{cell::UnsafeCell, ops::RangeInclusive};

extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end_inclusive: UnsafeCell<u64>;
}

/// Memory map
#[rustfmt::skip]
pub(super) mod map {

    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    /// Physical devices.
    pub mod mmio {
        use super::*;

        pub const START:            usize =         0x3F00_0000;
        pub const GPIO_START:       usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
    }

}


pub fn bss_range_inclusive() -> RangeInclusive<*mut u64> {
    let range: RangeInclusive<*mut u64>;
    unsafe {
        range = RangeInclusive::new(__bss_start.get(), __bss_end_inclusive.get());
	}
	range
}