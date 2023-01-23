
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


pub fn user_test() {
    let mut user1 = User {
        username: String::from("jiang"),
        email: String::from("xxx@gmail.com"),
        active: true,
        sign_in_count: 1,

    };
    println!("user1 username {}", user1.username);

    user1.username = String::from("jiangjiang");
    println!("user1 username {}, user1 email {}", user1.username, user1.email);

    println!("user1 active {}, user1 sign_in_count {},", user1.active, user1.sign_in_count);
}