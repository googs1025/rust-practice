mod user;

fn main() {

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };


    let res = rect1.area();
    println!("{:?}", res);


    let rect2 = Rectangle{
        width: 20,
        height: 50,
    };

    println!("{}", rect1.compare_area(rect2));

    user::user_test(); // 对外部文件引用

    let rect3 = Rectangle{
        width: 20,
        height: 50,
    };
    println!("{:#?}", rect3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        let a = self.width * self.height;
        return a;
    }

    fn compare_area(&self, other: Rectangle) -> bool {
        return self.area() > other.area();
    }


}
