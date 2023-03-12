use serde::{Serialize,Deserialize};
use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

#[derive(Serialize,Deserialize)]
pub struct UserModel<T>{  // 使用范型来扩展不同的输入
    pub user_id: T,
    pub user_name: String
}

// 自己实现Responder impl 的respond_to方法
// impl <'a> Responder <'a,'a> for UserModel {
//     fn respond_to(self, _: &Request) -> response::Result<'a> {
//         let json = serde_json::to_string(&self).unwrap();
//         Response::build()
//             .sized_body(json.len(), Cursor::new(json))
//             .header(ContentType::new("application", "json"))
//             .ok()
//     }
// }


