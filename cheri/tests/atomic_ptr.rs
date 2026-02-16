#![no_std]

extern crate alloc;
extern crate cheriot;

use core::sync::atomic::AtomicPtr;

#[no_mangle]
extern "C" fn test_atomic_ptr() -> i32 {
    let mut data = 5;
    let atomic_ptr = AtomicPtr::new(&mut data);
    assert_eq!(unsafe { *atomic_ptr.into_inner() }, 5);
    0
}
