use log::info;
use std::time::{self, Instant};
use tiny::Context;

//日志中间件demo
pub struct Logger;

impl Logger {
    pub fn logger(c: &mut Context) {
        let begin = time::Instant::now();
        info!("the path is:{}", &c.path);
        c.next();
        let cost = time::Instant::elapsed(&begin).as_millis();
        info!("cost time:{}", cost);
    }
}
