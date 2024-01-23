use std::os::raw::c_char;

extern "C" {
    pub fn Native_Print(str: *const c_char);
}

#[no_mangle]
pub extern "C" fn run_main(_argv: *const *const c_char, _argc: i32) -> i32 {
    let my_str = "Test boy!";
    unsafe {
        Native_Print(my_str.as_ptr() as _);
    }

    return 0;
}
