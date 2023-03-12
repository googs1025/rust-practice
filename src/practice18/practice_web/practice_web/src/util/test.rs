use mysql::*;
use mysql::prelude::*;




// Init_mysql 操作mysql，文档：https://github.com/blackbeam/rust-mysql-simple
// 测试mysql的使用
pub fn Init_mysql() {
    let dns = "mysql://root:123456@localhost:3306/test";
    let pool = Pool::new(dns).unwrap();
    // 使用Option枚举类元组捞出需要的结果
    let ret:Option<(i32,String)> = pool.get_conn()
        .unwrap()
        .query_first("select user_id, user_name from user where user_id=101")
        .unwrap();

    println!("{}", ret.unwrap().1);
}


