use crate::runtime_init;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
///
/// # Safety
///
/// - The `bss` section is not initialized yet. The code must not use or reference it in any way.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    runtime_init::runtime_init()
}