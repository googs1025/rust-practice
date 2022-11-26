fn main() {

    // 建立结构体
    let me = User{
        name: String::from("jiang"),
        age: 20,
    };

    me.version(); // 调用函数
    println!("{}", me.to_string());


    println!("名字:{}, 年龄:{}", me.name, me.age);
    println!("{:#?}", me);

    practice_array();
    practice_tuple();


}

// 结构体对象
#[derive(Debug)]
struct User{
    name: String,
    age: u8,
}

// 结构体对象的方法
impl User {
    fn version(&self){
        println!("1.000")
    }
    fn to_string(&self)->String{
        return String::from(format!("名字:{}, 年龄:{}", &self.name, &self.age));

    }
}


fn practice_array() {
    let tags:[&str;4] = ["java", "php", "js", "rust"];
    println!("{:?}", tags);

    for i in 0..tags.len() {
        println!("{:#?}", tags[i]);
    }
    // 迭代器遍历
    for item in tags.iter() {
        println!("{:#?}", item);
    }

    let aaa :[&str; 10] = ["";10];  // 空数组
    println!("{:?}", aaa);


    for (k, v) in tags.iter().enumerate() {
        println!("index: {}, value: {}", k, v);
    }

}

fn practice_tuple() {
    let my:(&str, u8) = ("jiang", 20);
    println!("{:#?}, {:#?}", my.0, my.1);
}