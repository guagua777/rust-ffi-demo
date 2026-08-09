

extern fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

// 外部库的c函数，库为extlib
#[link(name = "extlib")]
unsafe extern "C" {
   fn register_callback(cb: extern fn(i32)) -> i32;
   fn trigger_callback();
}

fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback(); // Triggers the callback.
    }
}