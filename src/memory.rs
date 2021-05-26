use core::ops::RangeInclusive;
use core::fmt::Display;

pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where T: From<u8>,
{
	let mut ptr = *range.start();
	let end_inclusive = *range.end();
	
	while ptr <= end_inclusive {
		core::ptr::write_volatile(ptr, T::from(0));
		ptr = ptr.offset(1);
	}
}

pub unsafe fn read_range<T>(range: RangeInclusive<*mut T>, max_entries: i32)
where T: From<u8> + core::fmt::Debug + Display,
{
    use crate::print;
    use crate::println;
    let mut ptr: *mut T = *range.start();
    let end_inclusive: *mut T = *range.end();

    let mut c = 0;

    while ptr <= end_inclusive && c < max_entries{
        let value: T = core::ptr::read_volatile(ptr);
        print!("Ptr: {:?} | ", ptr);
        println!("Value: {:?>0width$}", value, width=20);
        ptr = ptr.offset(1);
        c = c + 1;
    }

}