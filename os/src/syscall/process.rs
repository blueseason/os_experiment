//! App management syscalls
use crate::{
    task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_ms,
};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// print process info: task name and id
pub fn sys_process_info() -> isize {
    //    print_app_info();
    //    print_current_app(); //in batch.rs
    0
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_sbrk(size: i32) -> isize {
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
