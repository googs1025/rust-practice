fn main() {
    let a = 10;
    let b = 20;

    println!("max number: {}", max(&a, &b));
    println!("{}, {}", a, b)
}

// 注意生命周期的意思
fn max<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}
