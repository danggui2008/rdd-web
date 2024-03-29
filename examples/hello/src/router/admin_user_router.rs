use tiny::Engine;

use crate::{controller, middleware::auth};

pub(crate) fn route(mut r: Engine) -> Engine {
    //仅模使用，使用中间能力：中间组分为：全局中间件，分组中间件，单路由中间件
    let _admin_user_group = r
    //admin分组添加auth中间件：测试分组中间件能力
        .group("/admin").hooks(auth::Auth::auth)
        .get("/userinfo", controller::admin_user_controller::get_user);
    r
}
