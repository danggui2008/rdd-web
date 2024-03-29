use tiny::Context;

use crate::service::user_service;



pub fn get_user(c: &mut Context) {
    let user_id = "12345678"; //可以从Context上下文中获取
    let user = user_service::UserService::get_user_info(user_id).unwrap();
    c.json(user);
}