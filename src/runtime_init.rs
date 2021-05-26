use crate::{bsp, memory};

/// Reset BSS registers
#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bsp::memory::bss_range_inclusive());
}

/// Initialize 'main' kernel runtime
pub unsafe fn runtime_init() -> ! {

    zero_bss();

    crate::kernel_init()
}


/// Initialize other kernel runtime
pub unsafe fn runtime_init2() -> ! {
    zero_bss();

    crate::other_kernel_init()
}