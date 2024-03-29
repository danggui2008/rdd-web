use tiny::Engine;

use crate::controller;

pub(crate) fn route(r: Engine) -> Engine {
    //简单路由功能
    let r = r.get("/index", controller::index_controller::index);
    r
}
