use std::{collections::HashMap, fmt::Debug, str::FromStr};

use hyper::{
    header::{self, HeaderName, HeaderValue},
    Body, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{router::handler::Handler, BoxErr};

use super::{header::ExtractHeaderError, param::ExtractParamError, query::ExtractQueryError};
//上下文：为每一个请示创建上下文环境：主要包括req内容(已经解析出来),response，以及与此请求相关的
//handler列表
pub struct Context<'h, 'req> {
    pub params: HashMap<String, String>,
    pub forms: HashMap<String, String>,
    pub request: &'req Request<Body>,
    pub path: String,
    pub method: String,
    //handlers：路由匹配的handler，以及该路由对应的所有中间件
    //组成handlers列表（些列表已排序）：1）执全局中间件（如果有） 2）分组中间件（如果有）3）节点中间件（如果有）4）路由handler
    pub(crate) handlers: Vec<&'h Handler>,
    index: i32,
    pub response: Response<Body>,
}
impl<'h, 'req> Context<'h, 'req> {
    pub(crate) fn new(request: &'req Request<Body>) -> Self {
        Self {
            params: HashMap::new(),
            forms: HashMap::new(),
            handlers: Vec::new(),
            request,
            path: "".to_string(),
            method: "".to_string(),
            index: -1,
            response: Response::new(Body::default()),
        }
    }
}

impl<'h, 'req> Context<'h, 'req> {
    //请求对应执行链条：1）执全局中间件（如果有） 2）分组中间件（如果有）3）节点中间件（如果有）4）路由handler
     pub fn next(&mut self) {
        self.index += 1;
        let len = self.handlers.len() as i32;
        for index in 0..len {
            if index == self.index {
                if let Some(&handler) = &self.handlers.get(index as usize){
                    handler(self);
                }
                //序号+1
                self.index += 1;
            }
        }
    }
    //完成，主要要中间件中使用，一般在由于特别情况需要提前结束调用链。
    //例如：权限中间件在验证用户权限不足时，不需要再调用下游组件，提前结束调用链。
    pub fn done(&mut self){
        self.index = self.handlers.len() as i32;
    }
    pub fn header<T>(&self, name: &str) -> Result<T, ExtractHeaderError>
    where
        T: FromStr,
        T::Err: Into<BoxErr>,
    {
        super::header::header(self.request, name)
    }

    pub(crate) fn query<'de, T>(req: &'de Request<Body>) -> Result<T, ExtractQueryError>
    where
        T: Deserialize<'de>,
    {
        super::query::query(req)
    }

    pub fn param<T>(&self, name: &str) -> Result<T, ExtractParamError>
    where
        T: FromStr,
        T::Err: Into<BoxErr>,
    {
        super::param::param(&self.params, name)
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        let headers = self.response.headers_mut();
        headers.insert(
            HeaderName::from_str(name).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    pub fn json<T>(&mut self, json: T)
    where
        T: Serialize,
    {
        self.set_header(
            header::CONTENT_TYPE.as_str(),
            "application/json; charset=utf-8",
        );
        let data = serde_json::to_string(&json).unwrap();
        *self.response.body_mut() = Body::from(data.to_string());
    }

    pub fn string(&mut self, code: Option<StatusCode>, data: &str) {
        self.set_header(header::CONTENT_TYPE.as_str(), "text/plain; charset=utf-8");

        if let Some(code) = code {
            *self.response.status_mut() = code;
        } else {
            *self.response.status_mut() = StatusCode::OK;
        }

        *self.response.body_mut() = Body::from(data.to_string());
    }

    pub fn html(&mut self, data: &str) {
        self.set_header(header::CONTENT_TYPE.as_str(), "text/html; chartset=utf-8");
        *self.response.body_mut() = Body::from(data.to_string());
    }

    pub(crate) fn build_request(req: &Request<Body>) -> Context<'_, '_> {
        let mut context = Context::new(req);
        context.method = req.method().as_str().to_string();
        context.path = req.uri().path().to_string();
        context
    }
}

impl<'h, 'req> Debug for Context<'h, 'req> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("name", &self.method)
            .field("path", &self.path)
            .field("params", &self.params)
            .finish()
    }
}
