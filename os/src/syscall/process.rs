//! App management syscalls
use crate::batch::{print_current_app, run_next_app};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

/// print process info: task name and id
pub fn sys_process_info() -> isize {
    //    print_app_info();
    print_current_app();
    0
}
