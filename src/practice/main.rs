fn main(){
    // let myage=19; // 编译器自动判断
    let myage:u8=19; // 定义类型
    let youage=myage+2;
    println!("我的年龄:{}, 你的年龄:{}", myage, youage);

    let mut a = 123;
    println!("原本a: {}", a);
    a = 456;
    println!("后面a: {}", a);

    show_me()

}

fn show_me(){
    
    let my_name="jiangjiang";
    let my_age=25;

    println!("我的名字:{}, 我的年龄:{}", my_name, my_age)

}