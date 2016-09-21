use nuklear_sys::{nk_size, nk_handle};

use std::os::raw::c_void;
use std::mem;

pub unsafe extern "C" fn alloc(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
	/*if old.is_null() {
		free_rust_hacky(hnd, old);
	}*/
	
	trace!("allocating {} bytes", size);
	let size_size = mem::size_of::<nk_size>();
    let size = size + size_size;

    trace!("allocating {} / {} bytes", size_size, size);
	
	let mut v: Vec<u8> = Vec::with_capacity(size);
    
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    
    *(ptr as *mut nk_size) = size;
    ptr.offset(size_size as isize) as *mut c_void
}

pub unsafe extern "C" fn free(_: nk_handle, old: *mut c_void) {
    if old.is_null() {
    	trace!("no dealloc for empty pointer");
    	return;
    }

    let size_size = mem::size_of::<nk_size>();

    let old = old as *mut u8;
    let old = old.offset(-(size_size as isize));
    let old_size = *(old as *const nk_size);
    
    if old_size > 16_000_000_000 {
    	trace!("Invalid dealloc size {}", old_size);
    	return;
    }

    trace!("deallocating {} bytes", old_size);
	
	mem::drop(Vec::from_raw_parts(old, 0, old_size));
}