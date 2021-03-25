#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod print;
mod panic_wait;
mod runtime_init;
mod memory;
mod console;


unsafe fn kernel_init() -> ! {

    println!("[0] Hello from Rust!");

	panic!("Stopping here.")
}