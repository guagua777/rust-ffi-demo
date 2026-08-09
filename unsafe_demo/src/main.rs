

// https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#using-extern-functions-to-call-external-code
fn main() {
    println!("Hello, world!");

    let mut num = 5;


    // 普通引用和原始引用
    // 原始借用运算符
    // 创建一个引用，所以是借用  We call the action of creating a reference borrowing
    // the raw borrow operators: &raw const num creates a *const i32 immutable raw pointer, 
    // and &raw mut num creates a *mut i32 mutable raw pointer.
    let r1 = &raw const num;
    let r2 = &raw mut num;

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);

    unsafe {
        println!("r1: {:?}", *r1);
        *r2 += 1;

        println!("r2: {:?}", *r2);
    }
}



// 创建引用的行为，称为借用
// We call the action of creating a reference borrowing. 
// As in real life, if a person owns something, you can borrow it from them. 
// When you’re done, you have to give it back. You don’t own it.