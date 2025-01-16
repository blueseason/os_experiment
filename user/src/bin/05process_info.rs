#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::get_task_info;

#[no_mangle]
fn main() -> i32 {
    get_task_info();
    0
}
