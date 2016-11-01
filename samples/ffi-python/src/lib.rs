extern crate libc;

use std::ffi::CString;
use std::ptr;
use libc::{c_char, c_void, malloc, memset, strcpy, free};

#[no_mangle]
pub extern "C" fn get_checksum(filepath: *const c_char) -> *mut c_char {
    // Your code here
    if filepath == ptr::null() {
      return ptr::null_mut::<c_char>()
    }

    unsafe {
        let result = malloc(12);
        memset(result, 0, 12);
        strcpy(result as *mut c_char, CString::new("abcdef").unwrap().as_ptr());
        return result as *mut c_char;
    }
}

#[no_mangle]
pub extern "C" fn release_checksum(checksum: *const c_char) {
    unsafe {
        free(checksum as *mut c_void);
    }
}
