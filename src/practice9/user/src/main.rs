mod models;

fn main() {

    let mut u = models::user_model::new_user_model();
    models::user_model::set_user(&mut u);
    println!("{:#?}", u);

    let mut user_score_a = models::user_model::new_user_score_a();
    user_score_a.score = 10;
    user_score_a.user_id = 100;
    println!("{:#?}", user_score_a);

    let mut user_score_b = models::user_model::new_user_score_b();
    user_score_b.user_id = "jiangaaaa";
    user_score_b.score = 10000000000.0;
    println!("{:#?}", user_score_b);


    println!("user type: {:#?}", user_score_a.get_user_type());
    println!("user type: {:#?}", user_score_b.get_user_type());

    // 泛型使用在方法中
    println!("user id: {:#?}", user_score_a.get_user_id());
    println!("user id: {:#?}", user_score_b.get_user_id());

    let a = User{id: 23, name: "jiangjiang".to_string(), age: 18 };
    let b = a.clone(); // 用clone 深拷贝
    println!("a={:?}, b={:?}", a,b);

    // 实现default语意，就可以不用所有字段都初始化。
    let mut c = User{id: 111, ..Default::default()};
    println!("c={:?}, b={:?}", c,b);

    let res_user = load_user1(&mut c);
    println!("{:?}", res_user);

    let res_user1 = load_user2(c.clone());
    println!("{:?}", res_user1);


}

// 不使用clone语意，自己实现clone
// impl Clone for User {
//     fn clone(&self) -> Self {
//         User {
//             id: self.id,
//             name: self.name.clone() + "with_copy",
//             age: self.age,
//         }
//     }
// }


// #[derive(Debug, Default)]
#[derive(Debug, Default, Clone)] // String不能实现copy语意
struct User {
    id: i32,
    name: String,
    age: i32,

}

// 法一：可变引用
fn load_user1(u: &mut User) -> User {
    u.id = 22222;
    u.age = 1000;
    u.clone()

}

// 法二：直接传变量，不传引用
fn load_user2(mut u: User) -> User {
    u.age = 10000;
    u.clone()
}


