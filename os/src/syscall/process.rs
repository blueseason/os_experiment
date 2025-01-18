//! App management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};

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
