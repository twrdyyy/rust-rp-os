use cortex_a::asm;
pub use asm::nop;

/// Wait forever function
///
/// It loops with 'wait for event' operation
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}

