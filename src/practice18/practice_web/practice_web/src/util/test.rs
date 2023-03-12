use mysql::*;
use mysql::prelude::*;

// 引用同包，不同文件的方法
pub use crate::init_db::init_db;
use crate::util::db;

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

#[derive(Debug)]
struct UserModel {
    user_id: i32,
    user_name: String,
}

// List_user 捞出某类型的列表
pub fn List_user() {
    init_db(5, 10);
    let mut conn = db().unwrap();
    let sql = "select user_id, user_name from user";
    // 会输出一个list：[UserModel { user_id: 101, user_name: "jiang" }, UserModel { user_id: 111, user_name: "aaaa" }]
    let users = conn.query_map(sql, |(user_id, user_name)| UserModel {
        user_id,
        user_name,
    });
    println!("{:?}", users.unwrap());
}

// Pre_Select_user 使用准备
pub fn Pre_Select_user() {
    init_db(5,10);
    let mut conn = db().unwrap();
    let stmt = conn.prep("select user_id,user_name from user where user_id=?").unwrap();
    let ret: Option<(i32,String)> = conn.exec_first(&stmt, (111,)).unwrap();

    let ret2 = conn.exec_map(&stmt, (111,), |(user_id, user_name)| UserModel {
        user_id,
        user_name,
    }).unwrap();

    println!("{:?}", ret.unwrap());
    println!("{:?}", ret2);
}

// Insert_user 创建表中的数据
pub fn Insert_user() {
    init_db(5,10);
    let mut conn = db().unwrap();

    let stmt = conn.prep("insert into user(user_id,user_name) values(?,?)").unwrap();
    let ret = conn.exec_iter(stmt, (333, "jiangjiang")).unwrap();
    println!("{:?}", ret.affected_rows());
}


// Create_user 创建表中数据
pub fn Create_user() {
    init_db(5, 10);
    let mut conn = db().unwrap();
    let ret =
        "insert into user(user_id,user_name) values(?,?)".with((100,"hhhh")).run(&mut conn).unwrap();
    println!("{:?}",ret.affected_rows());


    let mut conn1 = db().unwrap();
    let sql = "select user_id, user_name from user";
    let users=conn1.query_map(sql, |(user_id, user_name)|{
        UserModel{
            user_id,
            user_name,
        }
    }).unwrap();

    println!("{:?}",users);

}
