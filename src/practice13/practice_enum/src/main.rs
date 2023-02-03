use std::intrinsics::likely;

fn main() {
    // println!("{:?}", Sex::Male(String::from("男"),1));
    // println!("{:?}", Sex::Female(String::from("女"),0));

    let u = User{
        id: 101,
        // sex: Sex::Male,
        sex: Sex::Male(String::from("男"), 1),
    };
    // println!("{:?}", u);
    // check_sex(u);
    // new_enum_ipaddr();
    try_message();
}

#[derive(Debug)]
enum Sex {
    Male(String,i32),
    Female(String,i32),
    // Male,
    // Female,
}

#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex
}

fn check_sex(u: User) {
    // if let Sex::Male = u.sex {
    //     println!("男性");
    // }

    // 取值
    if let Sex::Male(s,i) = u.sex {
        println!("{} {}", s,i);
    }


}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct  IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn new_enum_ipaddr() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let ip_four = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip_six = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route(IpAddrKind::V4);



}

fn route(ip_type: IpAddrKind)  {
    println!("{:?}", ip_type);

}


enum IpAddr1 {
    V4(String),
    V6(String),
    Other(u8, u8, u8, u8),
}

fn try_enum() {
    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));
    let other = IpAddr1::Other(127,0,0,1);
}



struct QuitMessage;
struct MoveMessage{
    x: i32,
    y: i32
}
struct WriteMessage (String);
struct ChangeColorMessage(i32,i32,i32);

fn try_message() {
    // enum实例化
    let bb = Message::Quit;
    let aa = Message::ChangeColor(1,2,3);
    let cc = Message::Write(String::from("jiangjiang"));
    let dd = Message::Move {
        x: 1,
        y: 2,
    };

    let m = Message::Move {
        x: 11111,
        y: 22222,
    };
    m.call();
    bb.call();
    cc.call();

}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // match 匹配使用
        match self {
            Message::Move {x, y} => {
                println!("move{}{}", x, y)
            }
            Message::Quit => {
                println!("quit")
            }
            Message::Write(x) => {
                println!("write{}", x)
            }
            _ => {} // 其他全没有，都要经过这
        }

    }
}

fn option_practice() {
    let some_number = Some(5);
    let some_string = Some("aaa");

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = Some(five);
    println!("{:?}", six);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

