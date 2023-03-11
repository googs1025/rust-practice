#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn users()->&'static str{
    "用户列表"
}

#[get("/users/<uid>")]
fn users_detail(uid:i32)->String{
    format!("user detail id:{}",uid)
}

#[launch]
fn rocket() -> _ {
    // 执行server
    rocket::build().mount("/", routes![index, users, users_detail])
}

