#![allow(dead_code)]

extern crate alloc;

use nuklear_sys::{nk_size, nk_handle};
use super::ALIGNMENT;

use self::alloc::heap;
use std::os::raw::c_void;
use std::mem;

pub unsafe extern "C" fn alloc(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
    trace!("allocating {} bytes", size);

    let size_size = mem::size_of::<nk_size>();
    let size = size + size_size;

    let memory = heap::allocate(size, ALIGNMENT);
    trace!("allocating {} / {} bytes", size_size, size);

    *(memory as *mut nk_size) = size;
    trace!("allocated {} bytes at {:p}", size, memory);
    memory.offset(size_size as isize) as *mut c_void
}

pub unsafe extern "C" fn free(_: nk_handle, old: *mut c_void) {
    if old.is_null() {
        return;
    }

    let size_size = mem::size_of::<nk_size>();

    let old = old as *mut u8;
    let old = old.offset(-(size_size as isize));
    let old_size = *(old as *const nk_size);

    trace!("deallocating {} bytes from {:p}", old_size, old);

    heap::deallocate(old as *mut u8, old_size, ALIGNMENT);
}
