use std::rc::Rc;

fn main() {

    let a = 5;  // 分配在栈上
    let aa = Box::new(5); // 分配在堆中
    let b = &a;   // 可以简单理解为取地址
    println!("{}", a==*b);
    println!("{}", a==*aa);

    practice()


}

fn print_test(s: &String) {
    println!("{}", s)
}

fn practice() {
    let mut a = String::from("jiang");
    print_test(&a);

    // 如果下面想继续对a变量操作，需要使用 "引用"
    a.push_str("jiangjiang");

    println!("{}", a);

}

fn print_test_Rc(s: Rc<String>) {
    println!("{}", s)
}

fn practice_Rc() {
    let a = Rc::new(String::from("jiang"));
    let b = a.clone();
    print_test_Rc(b);
    // 使用Rc不能获得 mut可变引用。

    println!("{}", a);

}


// Box 装箱：可以把原本放在栈上的变量，放在堆上，并在栈上创建是一个指向堆的指针结构。
// Rc：让一个值的多个所有者，调用clone产生一个指针指向该值。
// 1. 当Rc指针全部销毁，值也会销毁
// 2. 不能通过Rc获取可变引用
// 3. Rc只能在单线程使用，多线程要用Arc