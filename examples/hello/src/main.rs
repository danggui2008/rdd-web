use env_logger::Env;
use log::info;
use std::net::SocketAddr;
use tiny::{default,Server};

mod controller;
mod model;
mod router;
mod service;
mod middleware;
#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("C_LOG_LEVEL", "debug") 
        .write_style_or("C_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("env_logger initialized.");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //default()此方式会添加错误处理中间件，new()则不添加
    let mut r = default();
    //添加日志中间件;
    r = r.hooks(middleware::logger::Logger::logger);
    r = router::route::route(r);
    Server::run(addr, r).await;
}
//简要说明:
//a) 测试目的：路由能力
//输入：http://127.0.0.1:3000/index
//输出：hello world
//经过中间件：recovery(错误处理),logger（日志中间件）

//b)测试目的：路由"分组"能力
//输入：http://127.0.0.1:3000/user/info
//输出：{"name":"hmm","age":18}
//经过中间件：recovery(错误处理),logger（日志中间件）
//c)测试容错能力：panic情况
//输入：http://127.0.0.1:3000/user/info2
//输出：系统开小差
//经过中间件：recovery(错误处理),logger（日志中间件）

//d)测试中间件能力,这里主要模拟鉴权中间件
//输入：http://127.0.0.1:3000/admin/userinfo
//输出：权限不足
//经过中间件：recovery(错误处理),logger（日志中间件）,权限中间件


