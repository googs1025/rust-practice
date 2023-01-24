use crate::models::book_model::Book;
use crate::models::cellphone_model::Phone;

pub trait Prods {
    fn new(id:i32, price:f32) ->Self;
    fn get_price(&self) ->f32;
}

// 使用prods抽象类定义Book的get_price方法
impl Prods for Book {
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }

    fn new(id:i32, price:f32 ) -> Book {
        Book{
            id,
            price,
        }
    }
}

// 使用prods抽象类定义Book的get_price方法
impl Prods for Phone {
    fn get_price(&self) -> f32 {
        &self.price + 20.0
    }

    fn new(id:i32, price:f32 ) -> Phone {
        Phone{
            id,
            price,
        }
    }
}

impl std::ops::Add<Book> for Book {
    type Output = f32;
    fn add(self, rhs: Book) -> f32 {
        self.get_price() + rhs.get_price()
    }
    
}