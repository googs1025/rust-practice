#[derive(Debug)]
pub struct UserModel {
    user_id: i32,
    user_name: String,
    user_age: u8,
    user_tags: [&'static str;5]
}

pub fn new_user_model() -> UserModel {
    UserModel {
        user_id: 0,
        user_name: String::new(),
        user_age: 0,
        user_tags: ["";5]
    }
}

pub fn set_user(u:&mut UserModel) {
    u.user_id = 100;
    u.user_age = 10;
    u.user_tags = ["java", "python", "go", "rust", "c++"];
    u.user_name = String::from("jiangjaing");
}


/*

不用泛型的方式：如果id score类型不同，就需要写两种struct

#[derive(Debug)]
pub struct UserScoreA {
    pub user_id: i32,
    pub score: i32,
}

pub struct UserScoreB {
    pub user_id: f32,
    pub score: f32,
}

pub fn new_user_score() -> UserScore {
    UserScore {
        user_id: 0,
        score: 0,
    }
}

 */


// 使用泛型：
#[derive(Debug)]
pub struct UserScore<T, U> {
    pub user_id: T,
    pub score: U,
    pub kind: &'static str,
}

pub fn new_user_score_a() -> UserScore<i32, i32> {
    UserScore{
        user_id: 0,
        score: 0,
        kind: "A类用户",
    }
}

pub fn new_user_score_b() -> UserScore<&'static str, f32> {
    UserScore{
        user_id: "",
        score: 0.0,
        kind: "B类用户",
    }
}

impl <T,U> UserScore<T,U> {
    pub fn get_user_type(&self) -> &'static str{
        &self.kind
    }

    pub fn get_user_id(&self)  ->&T {
        &self.user_id
    }
}

