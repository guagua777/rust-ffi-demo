use std::fmt;

// https://doc.rust-lang.org/rust-by-example/std_misc/ffi.html

/// 步骤：
/// 1. 引入外部函数，并且指定链接库，且需要添加unsafe关键字，以及extern关键字和代码块
/// 2. 调用外部函数，且调用时需要添加unsafe关键字

// extern代码块，且该代码块必须带有#[link]注解
// 外部函数必须在带有#[link]注解的extern代码块内
// this extern block links to the libm library
// 特定系统才编译
#[cfg(target_family = "windows")]
// 链接库 微软 Visual C++ 运行时库）
#[link(name = "msvcrt")]
unsafe extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}
#[cfg(target_family = "unix")]
#[link(name = "m")]
unsafe extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    // 传入一个Complex，返回一个Complex
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
// 使用外部函数，且需要添加unsafe关键字
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // 实例化complex结构体，表示复数
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 调用外部方法，且需要添加unsafe关键字
    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    println!("cos({:?}) = {:?}", z, cos(z));
}

// Minimal implementation of single precision complex numbers
#[repr(C)] // 代表按照c语言的内存布局存储复杂数
#[derive(Clone, Copy)] // 代表可以复制和拷贝
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 这个0.是什么意思？0.0
        // 
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}