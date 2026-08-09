
/// 该库中的方法，该方法可以被c进行调用
#[unsafe(no_mangle)]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
