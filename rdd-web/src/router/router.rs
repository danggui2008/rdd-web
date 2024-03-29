use std::{collections::HashMap, fmt::Debug};

use log::trace;

use super::{handler::Handler, utils, Node};
use crate::Context;

//路由注册表
pub(crate) struct Router {
    //按照请求方法不同而分类的前缀树：GET前缀树,POST前缀树...
    pub node_tree: HashMap<String, Node>,
    //路由handler
    pub handlers: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            node_tree: HashMap::new(),
            handlers: HashMap::new(),
        }
    }
}

impl Router {
    pub(crate) fn add_route<H>(
        &mut self,
        method: &str,
        pattern: &str,
        group_id: Option<&str>,
        handler: H,
    ) where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        if ! pattern.starts_with('/'){
            panic!("路径必须以'/'开头:{}",pattern)
        }
        if self.node_tree.get_mut(method).is_none() {
            let node = Node::new();
            self.node_tree.insert(method.to_string(), node);
        }
        if let Some(node) = self.node_tree.get_mut(method) {
            let parts = utils::parse_pattern(pattern);
            node.insert(pattern, parts, group_id, 0);
            let key = format!("{}_{}", method, pattern);
            println!("key:{}", key);
            self.handlers.insert(key, Box::new(handler));
        }
    }

    //根据请示路径找到路由节点以及提取路径上的参数：如果节点信息为：/:lang/doc，用户待匹配路径为/c/doc
    //提输出（节点，（lang,c））
    pub(crate) fn get_route(
        &self,
        method: &str,
        path: &str,
    ) -> (Option<&Node>, HashMap<String, String>) {
        trace!("待查找的路由：{}",path);
        if let Some(root) = self.node_tree.get(method) {
            let path_parts = utils::parse_pattern(path);
            return Node::get_node_info_by_root_node(root, &path_parts);
        }
        (None, HashMap::new())
    }

    //给指定节点添加“节点中间件”
    pub(crate) fn add_hooks<H>(&mut self, pattern: &str, method: &str, handler: H)
    where
        H: Fn(&mut Context) + Send + Sync + 'static,
    {
        if let Some(root) = self.node_tree.get_mut(method) {
            let parts = utils::parse_pattern(&pattern);
            if let Some(node) = root.search_mut(&parts, 0) {
                if node.pattern.is_some() {
                    //添加中间件
                    node.middlewares.push(Box::new(handler));
                } else {
                    panic!("路径异常不能添加中间件")
                }
            }
        } else {
            panic!("路径异常不能添加中间件")
        }
    }
}
impl Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("node_tree", &self.node_tree)
            .field("handlers size", &self.handlers.len())
            .finish()
    }
}
