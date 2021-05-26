#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static CORE2_ID: u64 = 1;
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static CORE3_ID: u64 = 2;
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static CORE4_ID: u64 = 3;