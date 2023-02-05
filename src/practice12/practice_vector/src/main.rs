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

    // 返回数字最大结果
    let num_list = vec![23,45,67,8];
    // let res = largest_num(&num_list);
    let res = largest_generics(&num_list);
    println!("res: {}", res);


    let char_list = vec!['b', 'c', 'z', 'a'];
    // let res = largest_char(&char_list);
    let res = largest_generics(&char_list);
    println!("res char: {}", res);
}

fn largest_num(list: &[i32]) -> i32 {
    let mut largest_num = list[0];

    for &item in list.iter() {
        if item > largest_num {
            largest_num = item;
        }
    }

    largest_num
}

fn largest_char(list: &[char]) -> char {
    let mut largest_char = list[0];

    for &item in list.iter() {
        if item > largest_char {
            largest_char = item
        }
    }

    largest_char
}


// 使用泛型适配多种类型的输入与返回
fn largest_generics<T: std::cmp::PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

