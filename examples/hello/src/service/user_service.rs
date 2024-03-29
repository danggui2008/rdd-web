use std::convert::Infallible;

use crate::model::{self, user::User};

pub struct UserService;


impl  UserService{
    pub fn get_user_info(user_id:&str)->Result<User,Infallible>{
        //模拟从数据库中取出
        let user = model::user::User::new("hmm".to_string(), 18);
        Ok(user)
    }
}
