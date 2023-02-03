use proc_macro::bridge::LitKind::Str;
use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("red"), 100);
    scores.insert(String::from("blue"), 200);

    let a = 100;
    let b = String::from("try");
    let mut map2 = HashMap::new();
    map2.insert(a, b); // a b 的所有权都会转移

    // 取出key-value
    let team_name = String::from("red");
    let res = scores.get(&team_name);
    println!("{:?}", res);

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 有就不插入，没有才插入
    scores.entry(String::from("red")).or_insert(50);

}
