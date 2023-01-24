use crate::models::book_model::Book;

pub trait Stock {
    fn get_stock(&self) -> f32;
}

impl Stock for Book {
    fn get_stock(&self) -> f32 {
        12345.0
    }
}