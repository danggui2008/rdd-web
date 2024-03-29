use tiny::Engine;

use crate::controller;

pub(crate) fn route(mut r: Engine) -> Engine {
    //路由分组
    let _user_group = r
        .group("/user")
        .get("/info", controller::user_controller::get_user)
        //模拟panic
        .get("/info2", controller::user_controller::get_user2);
    r
}
