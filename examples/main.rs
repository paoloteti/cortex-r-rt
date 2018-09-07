#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_r_rt;
extern crate panic_abort;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    loop {}
}

