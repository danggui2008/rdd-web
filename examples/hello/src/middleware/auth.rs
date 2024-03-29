use tiny::Context;

//权限中间件demo
pub struct Auth;


impl Auth {
    pub fn auth(c: &mut Context) {
        //模拟,用户可以从Context获取真实token
        let token = c
            .header::<String>("token")
            .or_else(|e| Ok::<String, ()>("12345".to_string()))
            .unwrap();

        if "123456" == token.as_str() {
            //进行后面逻辑
            c.next();
        } else {
            c.done();
            c.string(None, "权限不足");
        }
    }
}
