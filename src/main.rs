#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language
//! How awesome is Rust? Even now I do not know

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(asm)]
#![feature(trait_alias)]
#![no_main]
#![no_std]
#![allow(clippy::upper_case_acronyms)]
#![feature(const_fn_fn_ptr_basics)]

use scheduler::SCHEDULER;

mod bsp;
mod exception;
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
mod quicksort;

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

/// Kernel initialization function
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

/// Other Kernel initialization
unsafe fn other_kernel_init() -> ! {
    println!("HELLO FROM THE SECOND THREAD");
    loop{}
}

/// Read line from console
fn read_line(result: &mut [u8; 100]) -> &str {
    use core::str::from_utf8;
    use console::interface::All;
    let mut c: char;
    
    let mut how_many_chars: usize=0;
    for (i, h) in result.iter_mut().enumerate()
    {
        how_many_chars=i;
        c=bsp::console::console().read_char();
        if c=='\n'
        {
            break;
        }
        *h=c as u8;
    }

    return from_utf8(&result[0..how_many_chars]).unwrap();
} 

/// Write string to console
fn write_str(_str: &str) {
    use console::interface::All;
    for c in _str.chars()
    {
        bsp::console::console().write_char(c);
    }
}

/// First task for scheduler demo
/// It sorts an array. :)
fn first_task() {
    use quicksort::quicksort as quicksort;
    println!("TASK 1 - SORTING");
    let mut x: [i32; 6] = [1, 3, 1, 0, 1, 2];
    println!("BEFORE:");
    for i in 0..5{
        print!("{} ", x[i]);
    }
    println!("\nAFTER:");
    quicksort(&mut x);
    for i in 0..5{
        print!("{} ", x[i]);
    }
    println!("\nEND OF TASK 1");
}

/// Second task for scheduler demo
/// This task just waits
fn second_task() {
    use core::time::Duration;
    use time::interface::TimeManager;
    println!("TASK 2 - WAITING");
    time::time_manager().spin_for(Duration::from_secs(1));
    println!("END OF TASK 2");
}

/// Interpreter main loop
///
/// Possible commands:
/// - sort -> sorts given array
/// - memory -> reveals first 100 registers of rust code in raspberry memory
/// - bss -> reveals bss memory
/// - wait -> waits for given amount of time
/// - quit -> quits interpreter
fn interpreter()
{
    let mut command;
    let mut buffer: [u8; 100]=[0; 100];
    loop
    {
        println!("Type a command: sort/wait/quit");
        command = read_line(&mut buffer);
        if command=="quit"
        {
            break;
        }
        else if command=="sort"
        {
            use quicksort::quicksort as quicksort;
            println!("Pass numbers you want to sort: ");
            let numbers_string = read_line(&mut buffer);
            let split_numbers = numbers_string.split(" ");
            let mut numbers_array = [9999; 100];
            let mut how_many_numbers = 0;
            for (i, number_string) in split_numbers.enumerate()
            {
                let _number = number_string.parse::<i32>().unwrap();
                numbers_array[i]=_number;
                how_many_numbers = i;
            }
            println!("BEFORE:");
            for i in 0..(how_many_numbers+1){
                print!("{} ", numbers_array[i]);
            }

            quicksort(&mut numbers_array);

            println!("\nAFTER:");
            for i in 0..(how_many_numbers+1){
                print!("{} ", numbers_array[i]);
            }
        }
        else if command == "wait"
        {
            println!("How long do you want to wait? [us]");
            let how_long_to_wait_string = read_line(&mut buffer);
            let how_long_to_wait = how_long_to_wait_string.parse::<u64>().unwrap();
            use core::time::Duration;
            use time::interface::TimeManager;
            debug!("Starting waiting");
            time::time_manager().spin_for(Duration::from_micros(how_long_to_wait));
            debug!("Finished waiting");
        }
        else if command == "bss"
        {
            println!("Reading bss memory...");
            unsafe {
                memory::read_range(bsp::memory::bss_range_inclusive(), 100);
            }
        }
        else if command == "memory"
        {
            println!("Reading memory...");
            println!("Rust code data (only first 100):");
            unsafe {
                memory::read_range(bsp::memory::rust_range_inclusive(), 100);
            }
            println!("Do you see core ids? :)");
        }
        else
        {
            println!("Unknown command {}", command);
        }
    }
    println!("You have quited interpreter");
}


/// The main function running after the early init.
unsafe fn kernel_main() -> ! {
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;
    use bsp::console::console;
    use console::interface::All;
    use cortex_a::{regs::*};
    println!("{}", OS_LOGO);
    println!("{:^37}", bsp::board_name());
    println!("Assert BSS is reset");
    memory::read_range(bsp::memory::bss_range_inclusive(), 100);
    println!();
    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("Booting on: {}", bsp::board_name());
    let (_, privilege_level) = exception::current_privilege_level();
    println!("Current privilege level: {}", privilege_level);
    exception::asynchronous::print_state();

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
    time::time_manager().spin_for(Duration::from_nanos(1));

    SCHEDULER.add_task(&(first_task as fn()->()));
    SCHEDULER.add_task(&(second_task as fn()->()));

    let mut task: fn() -> () = SCHEDULER.take_task().unwrap();
    task();
    SCHEDULER.add_task(&(second_task as fn()->()));
    task = SCHEDULER.take_task().unwrap();
    task();

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    write_str("Starting interpreter:\n");
    
    interpreter();

    loop {
    }

}