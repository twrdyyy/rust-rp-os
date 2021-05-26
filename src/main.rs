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


unsafe fn kernel_init2() -> ! {
    println!("HELLO FROM THE SECOND THREAD");
    loop{}
}


/*fn read_line(&mut result: &&[u8; 100]) -> &str {
    use core::str::from_utf8;
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
} */


fn first_task() {
    use quicksort::quicksort as quicksort;
    println!("ZADANIE 1 - SORTOWANIE");
    let mut x: [i32; 6] = [1, 3, 1, 0, 1, 2];
    println!("PRZED:\n");
    for i in 0..5{
        println!("{} \n", x[i]);
    }
    println!("PO:\n");
    quicksort(&mut x);
    for i in 0..5{
        println!("{} ", x[i]);
    }
}

fn second_task() {
    use core::time::Duration;
    use time::interface::TimeManager;
    println!("ZADANIE 2 - CZEKANIE PRZEZ 1 s");
    time::time_manager().spin_for(Duration::from_secs(1));
    println!("ZADANIE 2 KONIEC");
}
/// The main function running after the early init.
unsafe fn kernel_main() -> ! {
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;
    use bsp::console::console;
    use console::interface::All;
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

    scheduler::SCHEDULER.add_task(&(first_task as fn()->()));
    scheduler::SCHEDULER.add_task(&(second_task as fn()->()));

    let mut task: fn() -> () = scheduler::SCHEDULER.take_task().unwrap();
    task();
    scheduler::SCHEDULER.add_task(&(second_task as fn()->()));
    task = scheduler::SCHEDULER.take_task().unwrap();
    task();
    use cortex_a::asm;
    
    let ptr: *mut u32 = (0x40000090) as *mut _;
    println!("{:#010x}", first_task as  u32);
    core::ptr::write_volatile(ptr, first_task as  u32);
    asm::sev();
    println!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    //let mut buffer: [u8; 100]=[0;100];
    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
    
    
}