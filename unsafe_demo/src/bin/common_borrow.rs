fn main1() {
    let mut a = 5;

    let b = &a;

    println!("a: {}", a);

    let c = &mut a;

    *c = 10;

    println!("c: {}", c);
}

fn main() {
    let mut a = 5;

    let b = &a;

    println!("a: {}", a);

    let c = &mut a;

    *c = 10;

    println!("c: {}", c);

    unsafe {
        let a_p = &raw const a;
        println!("a_p: {}", *a_p);
    }
}
