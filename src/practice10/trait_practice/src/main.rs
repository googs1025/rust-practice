mod models;
mod api;
use models::book_model::*;
use models::cellphone_model::*;
use api::prods::Prods;
use api::stock::Stock;

fn main() {

    // let b = new_book(100, 100.0); // 特定对象的初始方法
    let b:Book = Prods::new(100, 100.0); // 使用trait接口定义通用的初始化方法
    println!("book: {:#?}", b);

    let price = b.get_price();
    println!("price: {}", price);

    show_prod_price1(b);
    // show_prod_price2(b);

    let cellphone:Phone = Prods::new(10, 10.5);
    println!("phone: {:#?}", cellphone);

    let price = cellphone.get_price();
    println!("price: {}", price);

    let cellphone1:Phone = Prods::new(10, 10.5);
    let b1:Book = Prods::new(100, 100.0);
    // let cellphone2:Phone = Prods::new(10, 10.5);
    let res = prod_price_sum(cellphone1, b1);
    println!("total price: {}", res);

    let b2:Book = Prods::new(100, 100.0);
    // println!("stock: {}", b2.get_stock());
    show_stock_detail(b2);




    let b3:Book = Prods::new(100, 100.0);
    let b4:Book = Prods::new(100, 100.0);
    prod_price_sum_add_reload(b3, b4);

}

fn show_prod_price1<T: Prods>(p: T) {
    println!("product price: {}", p.get_price())
}

fn show_prod_price2(p: impl Prods) {
    println!("product price: {}", p.get_price())
}

fn prod_price_sum<T: Prods, U: Prods>(p1: T, p2: U) -> f32 {
    p1.get_price() + p2.get_price()
}

fn prod_price_sum_add_reload(p1: Book, p2: Book) {
    println!("product total price: {}", p1 + p2)
}

// 注意这里泛型的写法。
fn show_stock_detail<T: Prods+Stock>(p:T) {
    println!("stock: {}", p.get_stock())
}