use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {

    // new_async();
    // use_channel();
    // practice_asynchronous();
    practice_synchronize()

}

fn new_async() {

    // 启动新线程
    let t = thread::spawn(print);
    println!("{}", "done!");
    t.join().unwrap(); // 等待线程结束。

    // 可以用闭包
    let t1 = thread::spawn(|| {
        for i in 0..5 {
            println!("{}", i);
            thread::sleep(Duration:: from_secs(1));

        }
    });
    t1.join().unwrap();

}

fn print() {
    for i in 0..5 {
        println!("{}", i);
        thread::sleep(Duration:: from_secs(1));
    }
}

fn use_channel() {

    let (tx, rx) = mpsc::channel();
    // move关键字可以让闭包使用外部变量。
    let t = thread::spawn( move || {
        for i in 0..6 {
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        }

    });

    // 法一： 只会接收一个。
    // let res = rx.recv().unwrap();
    // println!("{}", res);

    // 法二：可以loop打印很多个。
    // loop {
    //     let res = rx.recv();
    //     if let Ok(i)=res {
    //         println!("{}", i)
    //     } else {
    //         println!("{}", "done");
    //         break
    //     }
    // }

    // 法三： 可以直接遍历取值。
    for r in rx {
        println!("{}", r)
    }

    t.join().unwrap();

}

// practice_asynchronous 异步通道
fn practice_asynchronous() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn( move || {
        for i in 0..6 {
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        }

    });
    // 这里还是会一直发到send中。
    thread::sleep(Duration::from_secs(10));

    for r in rx {
        println!("{}", r)
    }
    t.join().unwrap(); // 等待线程结束。

}

// practice_synchronize 同步通道
fn practice_synchronize() {
    // 同步通道，可以设置数量
    let (tx, rx):(mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(3);

    let t = thread::spawn( move || {
        for i in 0..6 {
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        }

    });
    // 这里还是会一直发到send中。
    thread::sleep(Duration::from_secs(10));

    for r in rx {
        println!("{}", r)
    }
    t.join().unwrap(); // 等待线程结束。
}