use crate::Context;
use log::{debug, error, info};
use std::panic::{self, AssertUnwindSafe};

pub fn recovery(c: &mut Context) {
    //此处这样应该不行，对于panic问题应该结合中间件相关组件一起考虑，暂时先这样
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        debug!("recovery");
        c.next();
    }));
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("系统出错");
            c.string(None, "系统开小差");
        }
    }
}
