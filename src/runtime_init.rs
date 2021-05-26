use crate::{bsp, memory};

#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bsp::memory::bss_range_inclusive());
}

pub unsafe fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}


pub unsafe fn runtime_init2() -> ! {
    zero_bss();

    crate::kernel_init2()
}