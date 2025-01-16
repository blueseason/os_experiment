use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp",out(reg) fp);
    println!("== Begin stack trace ==");

    while fp != ptr::null() {
        let saved_ra = fp.sub(1);
        let saved_fp = fp.sub(2);
        println!(
            "Return address: {:p}, Old Stack Pointer: {:p}",
            saved_ra, saved_fp,
        );

        fp = saved_fp;
    }
    println!("== End stack trace ==");
}
