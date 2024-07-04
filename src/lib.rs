#[no_mangle]
pub fn get_string() -> *const u8 {
    b"Hello, C-World\0".as_ptr()
}