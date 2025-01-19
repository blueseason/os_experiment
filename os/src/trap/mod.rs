mod context;

use crate::{
    println,
    syscall::syscall,
    task::{self, exit_current_and_run_next, suspend_current_and_run_next},
    timer::set_next_trigger,
};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};
global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    // stvec有直接模式或向量模式，这里设置直接模式，地址为__alltraps
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    // 设置了 sie.stie 使得 S 特权级时钟中断不会被屏蔽
    unsafe {
        sie::set_stimer();
    }
}

/// handle an interrupt, exception, or system call from user space
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    task::user_time_end();
    let scause = scause::read(); //get trap cause
    let stval = stval::read(); //get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            //a7 系统调用编号， a0-a5 参数
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernal] IllegalInstrauction in application, kernel  killed it");
            exit_current_and_run_next();
            //            run_next_app();
        }

        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
            //run_next_app();
        }

        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }

        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    task::user_time_start();
    cx
}

pub use context::TrapContext;
