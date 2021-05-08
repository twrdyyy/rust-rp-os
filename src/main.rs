#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![feature(const_fn_fn_ptr_basics)]
mod bsp;
mod cpu;
mod print;
mod panic_wait;
mod runtime_init;
mod memory;
mod console;
mod synchronization;
mod driver;

unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use bsp::console::console;
    use console::interface::All;
    use driver::interface::DriverManager;

    println!("[0] Booting on: {}", bsp::board_name());

    println!("[1] Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        println!("      {}. {}", i + 1, driver.compatible());
    }

    println!(
        "[2] Chars written: {}",
        bsp::console::console().chars_written()
    );
    println!("[3] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
}