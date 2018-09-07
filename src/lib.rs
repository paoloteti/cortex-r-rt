//! Minimal startup routine for ARM Cortex-R
//!

#![crate_name = "cortex_r_rt"]
#![crate_type = "rlib"]
#![deny(warnings)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![no_std]

extern crate r0;

pub mod cpucore;
pub mod intvect;
pub mod silicon;

extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
    fn __pre_init();
}

extern "C" {
    static mut _sbss: u32;
    static mut _ebss: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sidata: u32;
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn cpu_reset() -> ! {
    cpucore::init_core_registers();
    cpucore::init_stack_pointers();

    #[cfg(feature = "errata57")]
    silicon::errata57();

    #[cfg(feature = "errata66")]
    silicon::errata66();

    __pre_init();

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    main(0, ::core::ptr::null());
    core::intrinsics::abort();
}

