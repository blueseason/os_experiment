use core::panic::PanicInfo;

#[cfg(not(test))]
use crate::{println, sbi::shutdown};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    /*    unsafe {
        print_stack_trace();
    }*/
    shutdown(true)
}

#[cfg(test)]
#[panic_handler]
fn panic_test(_: &PanicInfo) -> ! {
    loop {}
}
