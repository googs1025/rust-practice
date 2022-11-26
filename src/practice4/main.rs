fn main() {

    // practice_ptr1();
    // practice_ptr2();
    practice_string();


    // 建立数组
    let mut arr = Vec::new();
    arr.push(1);
    arr.push(2);

}

fn practice_ptr1() {

    let name = String::from("abc");
    // 查看容量方法，meta数据在栈上，具体内容在堆中。
    println!("长度:{}, 容量:{}", name.len(), name.capacity());
    println!("ptr is:{:p}", name.as_ptr());
    let a = name.as_ptr();

    let name2 = name; // 不会发生内存拷贝(不会拷贝成堆，只会出现两个栈指向同一个堆数据)
    println!("长度:{}, 容量:{}", name2.len(), name2.capacity());
    println!("ptr is:{:p}", name2.as_ptr()); // 两个地址值相同
    let b = name2.as_ptr();

    if a == b {
        println!("地址相同");
    } else {
        println!("地址不相同");
    }

}

fn practice_ptr2() {


    // ex1:
    let me = String::from("jiang");

    let you = String::from("jiang");
    // 两个地址不同，不同数据
    println!("ptr is {:p}", me.as_ptr());
    println!("ptr is {:p}", you.as_ptr());

    // ex2:
    let a = String::from("abc");
    let b = a; // 一旦赋值，a就没有用了，用了会报错
    println!("{}", b);

    // ex3:
    let aa = String::from("abc");
    let bb = aa.clone(); // 直接复制，开辟新内存堆
    println!("ptr is {:p}", aa.as_ptr());
    println!("ptr is {:p}", bb.as_ptr());

    // ex4:
    let aaa = String::from("abc");
    let bbb = &aaa; // 引用 &String，aaa 是 String的所有者，bbb是aaa的所有者
    println!("ptr is {:p}", aaa.as_ptr());
    println!("ptr is {:p}", bbb.as_ptr());

}

fn practice_string() {
    let mut me = String::from("abbbc");
    show_len(&me);
    println!("{}", me);
    // 注意：
    // println!("{}", me); 这是一个错误语句，把所有权给了show_len方法中的s，me变量就失效了。
    // 所以需要传递引用，不让所有权转移。

    // 需要改变的时候，要输入可变参数
    change(&mut me);
    println!("{}", me);

}

fn show_len(s: &String) {
    println!("{}", s.len());
}

fn change (s: &mut String) {
    s.push_str("_aaaa")
}
