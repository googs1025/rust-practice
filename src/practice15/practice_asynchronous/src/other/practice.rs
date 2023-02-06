use std::thread;
use std::time::Duration;

// 并发
pub fn practice1() {

    // 启动线程
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("num from new thread: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 0..6 {
        println!("num from main: {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    // 使用join() 方法 会阻塞，等待所有线程结束
    handle.join().unwrap();
}

// 并发+闭包用法
pub fn practice2() {

    let v = vec![1,2,3,4,5];

    // 使用闭包，需要用move语意 将外部变量放入闭包中。(会获取其所有权，不论是否为引用)
    let handle = thread::spawn( move || {
        println!("vector: {:?}", v)
    });

    handle.join().unwrap();

}