#![deny(warnings)]
#![no_std]
#![no_main]

pub mod batch;
#[macro_use]
mod console;
mod lang_item;
mod logging;
mod sbi;
mod stack_trace;
mod sync;
pub mod syscall;
pub mod trap;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

use core::arch::global_asm;
use log::*;

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x}]",
        stext as usize,
        etext as usize,
    );

    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    trap::init();
    batch::init();
    batch::run_next_app();
    //    sbi::shutdown(false)
    //    panic!("Shutdown machine");
}

fn clear_bss() {
    //extern “C” 引用一个外部的 C 函数接口
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // write_volatile避免编译器对写入顺序做优化
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/*
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}
*/
