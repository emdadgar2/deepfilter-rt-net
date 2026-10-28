use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

use crate::DeepFilterStream;

struct DfnHandle {
    stream: DeepFilterStream,
}

#[no_mangle]
pub extern "C" fn dfn_create(model_dir: *const c_char) -> *mut std::ffi::c_void {
    if model_dir.is_null() {
        return ptr::null_mut();
    }

    let path = unsafe {
        match CStr::from_ptr(model_dir).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };

    let stream = match DeepFilterStream::new(Path::new(path)) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let boxed = Box::new(DfnHandle { stream });
    Box::into_raw(boxed) as *mut std::ffi::c_void
}

#[no_mangle]
pub extern "C" fn dfn_process(
    handle: *mut std::ffi::c_void,
    input: *const f32,
    output: *mut f32,
    len: usize,
) -> usize {
    if handle.is_null() || input.is_null() || output.is_null() {
        return 0;
    }

    let dfn = unsafe { &mut *(handle as *mut DfnHandle) };
    let input_slice = unsafe { std::slice::from_raw_parts(input, len) };

    let produced = match dfn.stream.process(input_slice) {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let n = produced.len().min(len);
    if n > 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(produced.as_ptr(), output, n);
        }
    }

    n
}

#[no_mangle]
pub extern "C" fn dfn_destroy(handle: *mut std::ffi::c_void) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle as *mut DfnHandle));
    }
}
