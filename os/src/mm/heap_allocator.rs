//! The global allocator
use crate::{config::KERNEL_HEAP_SIZE, sync::UPSafeCell};
// rust 2024 defaut open the check if use static mut ref, eg: HEAP_SPACE
// or use inner mutable data structure
//#![allow(static_mut_refs)]

use buddy_system_allocator::LockedHeap;
use lazy_static::*;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

// 由于 HEAP_SPACE 是静态的并且被初始化为零（或未初始化），通常链接器会将它放在 BSS 段中。
// 或者通过 #[link_section = ".bss"] 指定
lazy_static! {
/// heap space ([u8; KERNEL_HEAP_SIZE])
static ref HEAP_SPACE: UPSafeCell<[u8; KERNEL_HEAP_SIZE]> =
    unsafe { UPSafeCell::new([0; KERNEL_HEAP_SIZE]) };
}
/// initiate heap allocator
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[allow(unused)]
pub fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        assert_eq!(*val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    println!("heap_test passed!");
}
