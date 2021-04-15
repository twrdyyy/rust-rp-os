#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod print;
mod panic_wait;
mod runtime_init;
mod memory;
mod console;
mod synchronization;

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
    kernel_main()
}


fn kernel_main() -> ! {
    use console::interface::Statistics;
    println!("{}", OS_LOGO);
    println!("[1] Chars written: {}", bsp::console::console().chars_written());
    println!("[2] Stopping here.");
    cpu::wait_forever()
}