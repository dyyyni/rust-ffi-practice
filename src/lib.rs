use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_string(end: u8) -> *const u8 {
    print!("You passed in an argument {}\n", end);
    b"Hello, C-World\n\0".as_ptr()
}

#[repr(C)]
pub struct RustString {
    ptr: *const c_char,
}

/// Uppercases the input string
/// 
/// # SAFETY
/// The input pointer needs to follow the same safety requirements
/// as Rust 'std::ffi::CStr::from_ptr' 
#[no_mangle]
pub unsafe extern "C" fn to_uppercase(input: *const c_char) -> RustString {
    let input = unsafe { std::ffi::CStr::from_ptr(input.cast()) };
    let input = input.to_string_lossy();
    
    let  mut output = input.to_uppercase();
    output.push('\n');
    output.push('\0');
    let boxed = output.into_boxed_str();
    RustString {
        ptr: Box::into_raw(boxed).cast(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_string(string: RustString) {
    let _ = Box::from_raw(string.ptr as * mut c_char);
}