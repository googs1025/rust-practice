fn get_user(uid:i32) -> &'static str {
    if uid == 101 {
        println!("user is :{}", "jiang");
        return "jiang"
    } else if uid == 102 {
        println!("user is :{}", "jiang11");
        return "jiang11"
    } else {
        println!("user is :{}", "jiang3344");
        return "jiang3344"
    }

    
}

fn main(){
    
    let name:&'static str="jiang"; // &str 类型
    println!("{}", name);

    let uid = 101;
    println!("{}", get_user(uid));

}