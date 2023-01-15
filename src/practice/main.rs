mod src;
mod lib;
// 默认寻找 lib.rs 或 lib/mod.rs 
fn main(){
    // let myage=19; // 编译器自动判断
    let myage:u8=19; // 定义类型
    let youage = myage + 2;
    println!("我的年龄:{}, 你的年龄:{}", myage, youage);

    let mut a = 123;
    println!("原本a: {}", a);
    a = 456;
    println!("后面a: {}", a);

    lib::show_me(); // 关键字:: 才能调用模块的方法

    lib::lib::helper(); // 调用别的模块
    
    src::src2::helper();
    src::config::show_version()
}

// fn show_me(){
    
//     let my_name="jiangjiang";
//     let my_age=25;

//     println!("我的名字:{}, 我的年龄:{}", my_name, my_age)

// }