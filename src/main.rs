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
mod time;

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
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;

    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("Booting on: {}", bsp::board_name());

    println!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    println!("Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        println!("      {}. {}", i + 1, driver.compatible());
    }

    // Test a failing timer case.
    time::time_manager().spin_for(Duration::from_nanos(1));

    loop {
        debug!("Spinning for 1 second");
        time::time_manager().spin_for(Duration::from_secs(1));
    }
}