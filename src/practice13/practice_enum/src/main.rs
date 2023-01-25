fn main() {
    println!("{:?}", Sex::Male(String::from("男"),1));
    println!("{:?}", Sex::Female(String::from("女"),0));

    let u = User{
        id: 101,
        // sex: Sex::Male,
        sex: Sex::Male(String::from("男"), 1),
    };
    // println!("{:?}", u);
    check_sex(u);
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

