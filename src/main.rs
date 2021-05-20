#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(asm)]
#![feature(trait_alias)]
#![no_main]
#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![feature(const_fn_fn_ptr_basics)]
mod bsp;
mod console;
mod cpu;
mod print;
mod panic_wait;
mod runtime_init;
mod memory;
mod synchronization;
mod driver;
mod time;
mod scheduler;

const OS_LOGO: &str = r#"
 _______                         __            _______   _______          ______    ______
/       \                       /  |          /       \ /       \        /      \  /      \
$$$$$$$  | __    __   _______  _$$ |_         $$$$$$$  |$$$$$$$  |      /$$$$$$  |/$$$$$$  |
$$ |__$$ |/  |  /  | /       |/ $$   |        $$ |__$$ |$$ |__$$ |      $$ |  $$ |$$ \__$$/
$$    $$< $$ |  $$ |/$$$$$$$/ $$$$$$/         $$    $$< $$    $$/       $$ |  $$ |$$      \
$$$$$$$  |$$ |  $$ |$$      \   $$ | __       $$$$$$$  |$$$$$$$/        $$ |  $$ | $$$$$$  |
$$ |  $$ |$$ \__$$ | $$$$$$  |  $$ |/  |      $$ |  $$ |$$ |            $$ \__$$ |/  \__$$ |
$$ |  $$ |$$    $$/ /     $$/   $$  $$/       $$ |  $$ |$$ |            $$    $$/ $$    $$/
$$/   $$/  $$$$$$/  $$$$$$$/     $$$$/        $$/   $$/ $$/              $$$$$$/   $$$$$$/
"#;


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

fn first_task() {
    println!("XD");
}

fn second_task() {
    use core::time::Duration;
    use time::interface::TimeManager;
    println!("XD2");
    time::time_manager().spin_for(Duration::from_secs(1));
    println!("XD2KONIEC");
}
/// The main function running after the early init.
unsafe fn kernel_main() -> ! {
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;
    use cortex_a::{regs::*};
    //use bsp::console::console;
    //use console::interface::All;
    println!("{}", OS_LOGO);
    println!("{:^37}", bsp::board_name());
    println!();
    println!("[ML]hello");

    
    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("Booting on: {}", bsp::board_name());
    println!("FREQUENCY: {}", CNTFRQ_EL0.get());
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


    //core::ptr::write_volatile(0xE0 as *mut fn()->(), first_task as fn()->());


    // Test a failing timer case.
    time::time_manager().spin_for(Duration::from_nanos(1));

    scheduler::SCHEDULER.add_task(&(first_task as fn()->()));
    scheduler::SCHEDULER.add_task(&(second_task as fn()->()));

    let mut task: fn() -> () = scheduler::SCHEDULER.take_task().unwrap();
    task();
    scheduler::SCHEDULER.add_task(&(second_task as fn()->()));
    task = scheduler::SCHEDULER.take_task().unwrap();
    task();
    //task = scheduler::SCHEDULER.take_task().unwrap();
    //task();
    loop {

    }
    
    
}