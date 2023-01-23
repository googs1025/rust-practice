fn main() {
    practice1();
    practice2();
    practice3();
}

fn practice1() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    const MAX_POINT: u32 = 100000;
    println!("The value of const is : {}", MAX_POINT);
}

fn practice2() {
    let x = 5;
    let x = x + 1;
    let x = x*2;
    println!("The value of x is : {}", x);
}

fn practice3() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("value of z is: {}", z);
}

fn practice4() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_hundred = x.1;

    let one = x.2;

    println!("value of one is: {}", one);
}