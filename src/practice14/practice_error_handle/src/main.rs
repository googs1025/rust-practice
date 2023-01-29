use std::fs;
use std::fs::File;
use std::io::{Error, Read};
use std::io::ErrorKind;

fn main() {

    // let res = test(1);
    // println!("{:?}", res);
    // // 使用if let 判断错误处理
    // if let Ok(s) = &res {
    //     println!("测试成功:{}", s)
    // }
    // if let Err(s) = &res {
    //     println!("测试失败:{}", s)
    // }
    // println!("Hello, world!");
    //
    // // unwrap
    // let res1 = test(1);
    // println!("{}", res1.unwrap());
    //
    // // expect
    // let res2 = test(1);
    // println!("{}", res2.expect("my error"));
    //
    // // unwrap_or
    // // 不panic 返回设置好的默认值
    // let res3 = test(11);
    // println!("{}", res3.unwrap_or("my default value".to_string()));
    //
    // // unwrap_or_else()
    // let res4 = test(11);
    // // 这里有闭包
    // println!("{}", res4.unwrap_or_else(|error|{
    //     error.to_string()
    // }));
    //
    // println!("-------------------------------------");
    //
    // let res_err = test_err1();
    // println!("{}", res_err.unwrap_or_else(|error|{
    //     error
    // }));
    //
    // let res_err = test_err2();
    // println!("{}", res_err.unwrap_or_else(|error|{
    //     error
    // }));
    //
    // println!("-------------------------------------");
    // let path = "./src/test.txt";
    // println!("start to read the file");
    // read_file(path.to_string());



    // read_file_1();
    // read_file_2();
    read_file_3();
}



fn test(i: i32) ->Result<String, i32> {
    // // 1. 调用panic宏
    // panic!("my panic!")

    if i < 10 {
        Ok(String::from("success"))
    } else {
        Err(0)
    }
}


fn step1() -> Result<String, String> {
    Ok("success".to_string())
}

fn step2() -> Result<String, String> {
    Err("err2".to_string()) // 故意一定会报错
}
/*
    test_err1
    test_err2
    两个方法是等价的。
 */

fn test_err1() -> Result<String, String> {
    let res1 = step1();
    if let Err(s) = res1 {
        Err("fail from step1".to_string())
    } else {
        let res2 = step2();
        if let Err(s) = res2 {
            Err("fail from step2".to_string())

        } else {
            Ok("done all step!".to_string())
        }
    }
}

// 错误传播
fn test_err2() -> Result<String, String> {
    step1()?; // 会直接返回
    step2()?;
    Ok("done all step!".to_string())

}


// 读取文件操作

fn read_file(path: String) {
    // 法一：
    let res = fs::read_to_string(&path).expect("read err");
    println!("{}", res);
    // 法二：
    let mut f = File::open(&path).expect("file err");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("read err");
    println!("{}", buf);
}

fn read_file_1() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("开始file 错误:{:?}", error)
        }
    };

}


fn read_file_2() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建file 错误:{:?}", e)

            },
            other_error=> panic!("开启file 错误:{:?}", other_error),
        }
    };

}

// 错误处理使用闭包的方法
fn read_file_3() {
    let f = File::open("test.txt").map_err(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("创建file 错误:{:?}", error);
            });
        } else {
            panic!("开启file 错误:{:?}", error)
        }
    });


}