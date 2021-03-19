#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

//! # Kernel library written in rust language

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
mod runtime_init;
mod memory;


unsafe fn kernel_init() -> ! {
	panic!()
}