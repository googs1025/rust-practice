mod models;

fn main() {

    let mut u = models::user_model::new_user_model();
    models::user_model::set_user(&mut u);
    println!("{:#?}", u);

    let mut user_score_a = models::user_model::new_user_score_a();
    user_score_a.score = 10;
    user_score_a.user_id = 100;
    println!("{:#?}", user_score_a);

    let mut user_score_b = models::user_model::new_user_score_b();
    user_score_b.user_id = "jiangaaaa";
    user_score_b.score = 10000000000.0;
    println!("{:#?}", user_score_b);


    println!("user type: {:#?}", user_score_a.get_user_type());
    println!("user type: {:#?}", user_score_b.get_user_type());

    // 泛型使用在方法中
    println!("user id: {:#?}", user_score_a.get_user_id());
    println!("user id: {:#?}", user_score_b.get_user_id());


}


