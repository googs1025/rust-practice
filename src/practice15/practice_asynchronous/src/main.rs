use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc,Arc, Mutex};
use futures::executor::block_on;
mod other;

fn main() {

    // new_async();
    // use_channel();
    // practice_asynchronous();
    // practice_synchronize();
    // practice_channel_send_struct_data();
    // practice_channel_send_struct_data_Arc();
    // test_curl();
    test_curl1();
    // test_share1();
    // test_mutex();
    // try_async();
    // other::practice::practice2();
    other::channel::practice_channel2();
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

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn new_user() -> User {

    return User{
        id: 111,
        name: String::from("jiang")
    }
}

fn practice_channel_send_struct_data() {
    // 设置channel发送 struct类型
    let (tx, rx):(mpsc::Sender<User>, mpsc::Receiver<User>) = mpsc::channel();
    let user = new_user();

    // 开始新线程，用闭包发送
    thread::spawn(move ||{
        tx.send(user).unwrap();
    });

    // 这里会发生value borrowed here after move 错误。因为send会拿走所有权。需要用ARC
    // println!("{:?}", user);

    // 接收
    println!("{:?}", rx.recv().unwrap());
}

fn practice_channel_send_struct_data_Arc() {
    // 设置channel发送 struct类型
    let (tx, rx):(mpsc::Sender<Arc<User>>, mpsc::Receiver<Arc<User>>) = mpsc::channel();
    let user = Arc::new(new_user());
    let u = user.clone();
    // 开始新线程，用闭包发送
    thread::spawn(move ||{
        tx.send(u).unwrap();
    });

    // 这里使用了Arc，user所有权不会转移
    println!("{:?}", user);

    // 接收
    println!("{:?}", rx.recv().unwrap());
}

fn curl(i :i32) {
    thread::sleep(Duration::from_secs(1));
    println!("第{}个网页显示", i)
}

fn test_curl() {
    // 只有一个线程
    // let t1 = thread::spawn(|| curl(1));
    // let t2 = thread::spawn(|| curl(2));
    // t1.join().unwrap();
    // t2.join().unwrap();


    // 循环join多线程
    let mut thread_pool = Vec::new();

    for i in 0..5 {
        // 多线程循环直接vec+闭包即可。
        thread_pool.push(thread::spawn( move || curl(i)));
    }
    for t in thread_pool {
        t.join().unwrap();
    }
}


fn curl1(i :i32, tx:mpsc::Sender<String>) {
    thread::sleep(Duration::from_secs(1));
    tx.send(format!("第{}个网页显示", i)).unwrap();
}

fn test_curl1() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        // 两种都行。
        // let clone_tx = tx.clone();
        let clone_tx= mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            curl1(i, clone_tx)
        });
    }

    thread::spawn(move || tx.send(String::from("开始抓取")).unwrap());

    for res in &rx {
        println!("{}", res)
    }

}

// 可以跟golang 的结果比较一下。
// 这里是不行的 值都是1 最后output是0，无法修改mut m 变量
fn test_share() {
    let mut m = 0;
    let mut pool = Vec::new();
    for _ in 0..15 {
        pool.push(thread::spawn( move ||{
            m = m + 1;
            println!("{}", m)
        }))
    }

    for t in pool {
        t.join().unwrap();
    }
    println!("{}", m)

}


// 多线程改变共享变量
// 生命周期变量 会贯穿整个生命周期。
static mut N: i32 = 0;

fn test_share1() {

    let mut pool = Vec::new();
    for _ in 0..15 {
        pool.push(thread::spawn( ||{
            unsafe {
                N = N + 1;
            }
        }))
    }

    for t in pool {
        t.join().unwrap();
    }
    unsafe {
        println!("{}", N);
    }

}

fn test_mutex() {
    let share_num = Arc::new(Mutex::new(0));
    let mut pool = Vec::new();

    for _ in 0..5 {
        let share_num_thread = share_num.clone();
        pool.push(thread::spawn(move || {
            let mut num = share_num_thread.lock().unwrap(); // 引用
            *num += 1; // 需要解引用 才能操作

        }))
    }

    for t in pool {
        t.join().unwrap();
    }

    println!("{:?}", share_num.lock().unwrap());

}

// 异步方法
async fn get_user() -> String {
    thread::sleep(Duration::from_secs(5));
    format!("user is {}, score is {}", "jiangjiang", get_score().await)
}

// 异步方法
async fn get_score() -> i32 {
   100
}

fn try_async() {
    let res = block_on(get_user());
    println!("{}", res);
    println!("{}", "done");
}
