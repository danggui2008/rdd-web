use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use log::{debug, info};

use super::engin::Engine;

pub struct Server {}

impl Server {
    pub async fn run(addr: SocketAddr, engin: Engine) {
        let engin = Arc::new(engin);
        debug!("路由注册表：{:#?}", &engin.router);
        let make_service = make_service_fn(|conn: &AddrStream| {
            let _addr = conn.remote_addr();
            let engin = engin.clone();
            let service = service_fn(move |req| {
                let engin = engin.clone();
                async move { Engine::handler(req, engin).await }
            });
            async move { Ok::<_, Infallible>(service) }
        });
        let server = hyper::server::Server::bind(&addr).serve(make_service);
        let graceful = server.with_graceful_shutdown(Server::graceful_shutdown());
        info!("启动成功，端口:{}",&addr);
        if let Err(err) = graceful.await {
            eprintln!("server error:{}", err);
        }
    }

    async fn graceful_shutdown() {
        tokio::signal::ctrl_c().await.expect("ctrl+c 关机失败");
    }
}
