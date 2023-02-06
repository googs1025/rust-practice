use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 单生产者，单消费者。
pub fn practice_channel() {

    // channel
    let (tx, rx) = mpsc::channel();

    // 开启线程发送data 给channel
    let handle = thread::spawn( move || {
        let value = String::from("test_channel");
        tx.send(value).unwrap();
        // 这里value的所有权转移了，不能再使用。

        let values = vec![
            String::from("aaaa"),
            String::from("aaaa"),
            String::from("aaaa"),
            String::from("aaaa"),
        ];
        // 遍历发送
        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 从channel 接收data
    // let result = rx.recv().unwrap();
    for res in rx {
        println!("res: {}", res);
    }
}

// 多生产者，单一消费者。
pub fn practice_channel2() {

    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        // 使用clone 来复制chan给其他线程。
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {

            let values = vec![
                String::from("aaaa"),
                String::from("aaaa"),
                String::from("aaaa"),
                String::from("aaaa"),
            ];
            // 遍历发送
            for val in values {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // 需要传主对象tx，不然会一直阻塞
    thread::spawn(move || tx.send(String::from("开始执行")).unwrap());

    for res in rx {
        println!("res: {}", res);
    }


}