fn main() {
    practice_if()
}



fn practice_if() {
    let uid:i32 = 100;
    println!("{}", uid);
    let a = 1;  // 语句
    // 当a==1，则b直接附值
    let b= if a==1{
        5
    } else {
        10
    };
    println!("{}", b);
    // aaa = add(uid);
    // println!("{}", aaa);

    let mut money = Coin::Penny;
    let res = value_in_cents(money);
    println!("{}", res);

    let mut money2 = Coin::Quarter(String::from("aaaa!"));
    let res = value_in_cents(money2);
    println!("{}", res);


    // 遍历enum类型
    for c in Coin::COIN {
        let aaa = value_in_cents1(c);
        println!("{}", aaa);
    }
}

// 使用match 匹配enum
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin:: Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {}", state);
            25
        }
    }
}

// 使用if let
fn value_in_cents1(coin: Coin) -> u32 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {}", state);
    } else {
        count += 1;

    }
    count
}

use Coin::*;

impl Coin {
    const COIN: [Self; 3] = [Penny, Nickel, Dime];
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}