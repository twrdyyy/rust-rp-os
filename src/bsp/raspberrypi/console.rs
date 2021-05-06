
//use crate::{synchronization, console, synchronization::NullLock};
use super::memory;
use crate::{bsp::device_driver, console};

//use crate::{bsp::device_driver};
use core::fmt;
//use synchronization::interface::Mutex;
/*
/// The mutex protected part.
struct QEMUOutputInner {
    chars_written: usize,
}

pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}


static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();


impl QEMUOutputInner {
    const fn new() -> QEMUOutputInner {
        QEMUOutputInner { chars_written: 0 }
    }

    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }

        self.chars_written += 1;
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.write_char('\r')
            }

            self.write_char(c);
        }

        Ok(())
    }
}

impl QEMUOutput {
    pub const fn new() -> QEMUOutput {
        QEMUOutput {
            inner: NullLock::new(QEMUOutputInner::new()),
        }
    }
}
*/
pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut panic_gpio = device_driver::PanicGPIO::new(memory::map::mmio::GPIO_START);
    let mut panic_uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_START);
    panic_gpio.map_pl011_uart();
    panic_uart.init();
    panic_uart
}

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}
/*
impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
       
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}
*/