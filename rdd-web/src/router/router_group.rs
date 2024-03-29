use hyper::Method;

use crate::{router::handler::Handler, Context, Engine};

pub struct RouterGroup<'r> {
    pub(crate) prefix: String,
    engin: &'r mut Engine,
}

impl<'r> RouterGroup<'r> {
    pub(crate) fn new(prefix: &str, engin: &'r mut Engine) -> Self {
        Self {
            prefix: prefix.to_string(),
            engin,
        }
    }

    fn add_route<H>(&mut self, method: &str, sub_pattern: &str, handler: H)
    where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        let pattern = format!("{}{}", self.prefix, sub_pattern);
        self.engin.router.add_route(
            method,
            pattern.as_str(),
            Some(self.prefix.as_str()),
            handler,
        );
    }

    pub fn get<S, H>(mut self, sub_pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::GET.as_str(), sub_pattern.as_ref(), handler);
        self
    }
    pub fn post<S, H>(mut self, sub_pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::POST.as_str(), sub_pattern.as_ref(), handler);
        self
    }
    pub fn put<S, H>(mut self, sub_pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::PUT.as_str(), sub_pattern.as_ref(), handler);
        self
    }

    pub fn delete<S, H>(mut self, sub_pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::DELETE.as_str(), sub_pattern.as_ref(), handler);
        self
    }
    pub fn patch<S, H>(mut self, sub_pattern: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        self.add_route(Method::PATCH.as_str(), sub_pattern.as_ref(), handler);
        self
    }

    //添加中间件
    pub fn hooks<H>(mut self, handler: H) -> Self
    where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        if let Some(md_vec) = self.engin.groups.get_mut(&self.prefix) {
            md_vec.push(Box::new(handler));
        } else {
            let mut middlewares: Vec<Box<Handler>> = Vec::new();
            middlewares.push(Box::new(handler));
            self.engin.groups.insert(self.prefix.clone(), middlewares);
        }
        self
    }
    //给指定的（路径，方法）添加中间件
    pub fn add_hooks<S, H>(self, sub_pattern: S, method: S, handler: H) -> Self
    where
        S: AsRef<str>,
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        let pattern = format!("{}{}", &self.prefix, sub_pattern.as_ref());
        self.engin
            .router
            .add_hooks(pattern.as_str(), method.as_ref(), handler);
        self
    }
    //路由分组
    pub fn group<S>(&mut self, prefix: S) -> RouterGroup
    where
        S: AsRef<str>,
    {
        let new_prefix = format!("{}{}", &self.prefix, prefix.as_ref());
        self.engin
            .groups
            .insert(prefix.as_ref().to_string(), Vec::new());
        RouterGroup::new(new_prefix.as_str(), self.engin)
    }
}
