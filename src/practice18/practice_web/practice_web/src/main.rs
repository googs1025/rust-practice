// #![feature(in_band_lifetimes)]
#[macro_use] extern crate rocket;

mod models;
mod util;

use mysql::prelude::Queryable;
use models::UserModel;
use models::Json;
use util::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users?<page>&<size>")]
fn users(page:Option<i32>, size:Option<i32>)->String{
    let get_page = page.unwrap_or(1);
    let get_size = size.unwrap_or(10);
    format!("用户列表,page:{},size:{}", get_page, get_size)

}

#[get("/users/<uid>", rank=1)]
fn users_detail_int(uid:i32)-> Json<UserModel<i32>>{
    Json(UserModel{user_id:uid, user_name:String::from("test")})
    // let user = UserModel{user_id:uid, user_name:String::from("test")};
    // return user
    // 这里最后还是转成str类型，但是json样貌。
    // serde_json::to_string(&user).unwrap()
}

#[get("/users/<uid>", rank=2)]
fn user_detail_str(uid:String)-> Json<UserModel<String>> {
    Json(UserModel{user_id:uid, user_name:String::from("String-test")})
}


// #[post("/users",format="json",data="<user>")]
// fn user_create(user:Json<UserModel<i32>>)-> Json<UserModel<i32>>{
//     println!("{},{}",user.user_name,user.user_id);
//     user
// }


#[launch]
fn rocket() -> _ {
    // 测试用
    // util::Init_mysql();
    // util::List_user();
    // util::Create_user();
    // util::Pre_Select_user();
    util::Insert_user();

    // init_db(5, 10); // 初始化连接池
    // let mut conn = db(); // 拿到db的连接实例
    // let ret: Option<(i32, String)> = conn.unwrap()
    //     .query_first("select user_id,user_name from user order by user_id desc limit 1 ")
    //     .unwrap();
    // println!("{:?}", ret.unwrap().1);

    // 执行server
    rocket::build().mount("/", routes![index, users, users_detail_int, user_detail_str])
}
