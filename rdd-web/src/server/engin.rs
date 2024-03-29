use std::{collections::HashMap, convert::Infallible, sync::Arc};

use hyper::{Body, Method, Request, Response};
use log::{debug, trace};

use crate::{
    middleware::recovery::recovery,
    router::{handler::Handler, router::Router},
    Context, RouterGroup,
};
//web处理引擎（其实代码安全可以移入Router），req参数简单解析
pub struct Engine {
    pub(crate) router: Router,
    pub(crate) groups: HashMap<String, Vec<Box<Handler>>>,
    //全局中间件
    pub(crate) middlewares: Vec<Box<Handler>>,
}

pub fn new() -> Engine {
    Engine {
        router: Router::new(),
        groups: HashMap::new(),
        middlewares: Vec::new(),
    }
}
pub fn default() -> Engine {
    let e = Engine {
        router: Router::new(),
        groups: HashMap::new(),
        middlewares: Vec::new(),
    };
    e.hooks(recovery)
}

impl Engine {
    //web请求入口：1）解析请求(req) 2）封装上下文参数(context)
    pub async fn handler(
        req: Request<Body>,
        engin: Arc<Engine>,
    ) -> Result<Response<Body>, Infallible> {
        let mut context = Context::build_request(&req);
        let (node, params) = engin
            .router
            .get_route(req.method().as_str(), req.uri().path());
        trace!("路径中的参数：{:#?}", &params);
        context.params = params;

        //添加全局中间件
        let mut middlewares = engin.get_middlewares();
        if let Some(node) = node {
            debug!("请示对应的路由节点:{:#?}", &node);
            let key = format!(
                "{}_{}",
                req.method().as_str(),
                node.pattern.as_ref().unwrap()
            );
            if let Some(group_id) = &node.group_id {
                //添加分组中间件
                middlewares.extend(engin.get_middlewares_by_group_id(group_id));
            }
            if let Some(handler) = engin.router.handlers.get(&key) {
                if node.middlewares.len() > 0 {
                    //添加节点本身的中间件
                    middlewares.extend(node.get_middlewares());
                }
                //添加路由本身handler
                middlewares.push(handler.as_ref());
                //设置本次上下文能执行的handler
                context.handlers.extend(middlewares);
            }
            
            debug!("请求上下文:{:#?}", &context);
            //执行用户业务逻辑handler
            context.next();
            //返回结果（响应）
            return Ok(context.response);
        } else {
            //执行中间件功能：主要执行全局中间件功能：日志中间件
            context.next();
            return Ok(Response::new(Body::from("404 not found")));
        }
    }

    fn add_route<H>(&mut self, method: &str, pattern: &str, handler: H)
    where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.router.add_route(method, pattern, None, handler);
    }

    pub fn get<S, H>(mut self, pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::GET.as_str(), pattern.as_ref(), handler);
        self
    }

    pub fn post<S, H>(mut self, pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::POST.as_str(), pattern.as_ref(), handler);
        self
    }

    pub fn put<S, H>(mut self, pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::PUT.as_str(), pattern.as_ref(), handler);
        self
    }

    pub fn delete<S, H>(mut self, pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::DELETE.as_str(), pattern.as_ref(), handler);
        self
    }

    pub fn patch<S, H>(mut self, pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::PATCH.as_str(), pattern.as_ref(), handler);
        self
    }

    //路由分组
    pub fn group<S>(&mut self, prefix: S) -> RouterGroup
    where
        S: AsRef<str>,
    {
        self.groups.insert(prefix.as_ref().to_string(), Vec::new());
        RouterGroup::new(prefix.as_ref(), self)
    }
    //添加中间件
    pub fn hooks<H>(mut self, handler: H) -> Self
    where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.middlewares.push(Box::new(handler));
        self
    }

    //为单个路由添加中间件
    pub fn add_hooks<S, H>(mut self, pattern: S, method: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.router
            .add_hooks(pattern.as_ref(), method.as_ref(), handler);
        self
    }

    fn get_middlewares_by_group_id(&self, group_id: &str) -> Vec<&Handler> {
        let mut middlewares: Vec<&Handler> = Vec::new();
        if let Some(handlers) = self.groups.get(group_id) {
            for handler in handlers.iter() {
                middlewares.push(handler.as_ref());
            }
        }
        middlewares
    }

    fn get_middlewares(&self) -> Vec<&Handler> {
        let mut middlewares: Vec<&Handler> = Vec::with_capacity(self.middlewares.len());
        for handler in self.middlewares.iter() {
            middlewares.push(handler.as_ref());
        }
        middlewares
    }
}
