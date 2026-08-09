



// 我们还可以使用 extern 来创建一个接口，允许其他语言调用 Rust 函数
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


fn main() {
    
}