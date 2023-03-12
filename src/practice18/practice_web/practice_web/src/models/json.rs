use serde::{Serialize};
use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

// 通用 转换成json的方式
pub struct Json<T>(pub T);

// 实现json 需要实现Responder impl 与 respond_to方法
impl<'a,T:Serialize> Responder<'a,'a> for Json<T> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let json= serde_json::to_string(&self.0).unwrap();
        Response::build()
            .sized_body(json.len(),Cursor::new(json))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}