fn main() {

    let aaa = new_normal_user(1,"jiang".to_string(), 20, false, "".to_string());
    let bbb = new_admin_user(1,"admin".to_string(), true, "".to_string());


    // 不会有所有权，所以不能返回一个值出来
    let res = aaa.print_user();
    println!("{}", res);

    println!("{}", bbb.print_user());

}



// 定义Print接口
pub trait Print {
    fn print_user(&self) -> String;
}

// 普通用户对象
pub struct NormalUser {
    pub name: String,
    pub id: i32,
    pub description: String,
    pub age: i32,
    pub operation: bool,
}

fn new_normal_user(id: i32, name: String, age: i32, operation: bool, description: String) -> NormalUser {
    NormalUser{
        name,
        id,
        age,
        operation,
        description,
    }
}

impl Print for NormalUser {
    fn print_user(&self) -> String {
        format!("name:{}, id:{}, 是否有admin操作权限:{}", self.name, self.id, self.operation)
    }
}

pub struct AdminUser {
    pub name: String,
    pub id: i32,
    pub description: String,
    pub operation: bool,
}

fn new_admin_user(id: i32, name: String, operation: bool, description: String) -> AdminUser {
    AdminUser{
        id,
        name,
        operation,
        description,
    }
}

impl Print for AdminUser {
    fn print_user(&self) -> String {
        format!("name:{}, id:{}, 是否有admin操作权限:{}", self.name, self.id, self.operation)
    }
}



