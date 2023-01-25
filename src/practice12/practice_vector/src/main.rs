/* 数组 vs 切片：
    1. 存在栈上
    2. rust内置类型
    3. 长度固定

    vector则相反。


*/

fn main() {
    let mut  tags = vec!["go", "java"];
    tags.push("c++");
    println!("tags {:#?}", tags);

    let mut tags1 = Vec::new();
    tags1.push("c++");
    tags1.push("c++");
    tags1.push("c++");
    tags1.push("c++");
    println!("tags1 {:#?}", tags1);

    // 遍历
    for i in 0..tags.len() {
        println!("{:?}", tags[i])
    }
    // 使用借用遍历，如果不用借用，后面不能再用tags变量
    for i in &tags {
        println!("{:?}", i)
    }
    println!("tags {:#?}", tags);

    let mut tags2:Vec<i32> = Vec::new();
    tags2.push(3);
    tags2.push(3);
    tags2.push(5);
    tags2.push(7);
    for i in &mut tags2 {
        println!("{:?}", i);
        *i = *i + 10;
        println!("{:?}", i);
    }

}


