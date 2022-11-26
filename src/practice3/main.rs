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
}

// fn add(i:i32) ->i32 {
//     i+1
//     return i;
// }