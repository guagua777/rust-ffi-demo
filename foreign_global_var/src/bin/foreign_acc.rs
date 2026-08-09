use std::ffi::CString;
use std::ptr;

#[link(name = "readline")]
unsafe extern "C" {
    // *const libc::c_char 只读指针
    // mut rl_prompt 可变
    static mut rl_prompt: *const libc::c_char;
}

fn main() {
    let prompt = CString::new("[my-awesome-shell] $").unwrap();
    unsafe {
        rl_prompt = prompt.as_ptr();

        println!("{:?}", rl_prompt);

        rl_prompt = ptr::null();
    }
}