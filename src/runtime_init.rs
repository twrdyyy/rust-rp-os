use crate::{bsp, memory};
#[no_mangle]
pub unsafe fn runtime_init() -> ! {
	memory::zero_volatile(bsp::memory::bss_range_inclusive());
	crate::kernel_init()
}