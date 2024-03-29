use tiny::Engine;

pub(crate) fn route(r: Engine) -> Engine {
    //index不需要鉴权与登录
    let mut r = super::index_router::route(r);
    r = super::user_router::route(r);
    //需要鉴权通过
    r = super::admin_user_router::route(r);
    r
}
