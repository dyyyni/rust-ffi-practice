#[no_mangle]
pub fn get_string(end: u8) -> *const u8 {
    print!("You passed in an argument {}\n", end);
    b"Hello, C-World\n\0".as_ptr()
}