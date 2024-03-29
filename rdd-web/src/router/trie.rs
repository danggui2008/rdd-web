use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::{handler::Handler, utils};
/*
 *前缀树的简单实现：前缀树是一个简单的支持动态路由：如：/hello/:name，则能匹配：/hello/zs,hello/hmm
 *目前前缀树支持：
 *1）静态路由：/user/list,/user/index
 *2) 参数匹配':'：/hello/:name，则能匹配：/hello/zs,hello/hmm
 *3）通配符'*':/static/\*filepath  则能匹配：/static/zzz.js,
 *1、构建前缀树：根据用户的路由路径构建前缀树
 */

pub(crate) struct Node {
    pub pattern: Option<String>,        // 待匹配路由，例如 /p/:lang
    pub part: Option<String>,           // 路由中的一部分，例如 :lang
    pub children: Vec<Node>,            // 子节点，例如 [doc, tutorial, intro]
    pub is_wild: bool,                  // 是否精确匹配，part 含有 : 或 * 时为true
    pub middlewares: Vec<Box<Handler>>, //节点中间单件
    pub group_id: Option<String>,       //其实就是分组前缀
}

impl Node {
    pub fn new() -> Self {
        Self {
            pattern: None,
            part: None,
            children: Vec::new(),
            is_wild: false,
            middlewares: Vec::new(),
            group_id: None,
        }
    }
}

impl Node {
    //在node孩子节点中查找是否有孩子匹配上，该方法用于节点插入
    pub fn match_child_mut(&mut self, part: &str) -> Option<&mut Node> {
        for node in self.children.iter_mut() {
            if node.is_wild {
                return Some(node);
            }
            if let Some(p) = &node.part {
                if p == part {
                    return Some(node);
                }
            }
        }
        None
    }

    //在node孩子节点中查找是否有孩子匹配上，把所有能匹配上的节点都找出来
    fn match_children(&self, part: &str) -> Vec<&Node> {
        let mut children = Vec::new();
        for node in &self.children {
            if node.is_wild {
                children.push(node);
                continue;
            }
            if let Some(p) = &node.part {
                if p == part {
                    children.push(node);
                }
            }
        }
        return children;
    }

    //在node孩子节点中查找是否有孩子匹配上，把所有能匹配上的节点都找出来
    fn match_children_mut(&mut self, part: &str) -> Vec<&mut Node> {
        let mut children = Vec::new();
        for node in self.children.iter_mut() {
            if node.is_wild {
                children.push(node);
                continue;
            }
            if let Some(p) = &node.part {
                if p == part {
                    children.push(node);
                }
            }
        }
        return children;
    }

    //构建前缀树
    pub fn insert(
        &mut self,
        pattern: &str,
        parts: Vec<&str>,
        group_id: Option<&str>,
        height: usize,
    ) {
        if parts.len() == height {
            self.pattern = Some(pattern.to_string());
            return;
        }
        let part = parts.get(height);
        if let Some(&part) = part {
            let child = self.match_child_mut(part);
            if let Some(child) = child {
                // 如果节点已经存在，递归深度遍历
                child.insert(pattern, parts, group_id, height + 1);
            } else {
                //节点不存在，新增节点
                let mut child = Node::new();
                child.part = Some(part.to_string());
                child.is_wild = Node::is_wild(part);
                //如果路由节点在路由分组中，则登记所在的分组
                if let Some(group_id) = group_id {
                    child.group_id = Some(group_id.to_string());
                }
                let len = self.children.len();
                self.children.insert(len, child);
                //？？重新取出继续深度遍历，新增节点
                let child = self.children.get_mut(len);
                if let Some(child) = child {
                    child.insert(pattern, parts, group_id, height + 1);
                }
            }
        }
    }

    //树遍历：根据真实路径片段（url片段）查询路中是否有匹配的节点
    pub fn search(&self, parts: &Vec<&str>, height: usize) -> Option<&Node> {
        if let Some(part) = &self.part {
            // "*"前缀要特殊处理
            if parts.len() == height || part.starts_with('*') {
                //只有pattern有值的才是合法节点
                let node = match self.pattern {
                    Some(_) => Some(self),
                    None => None,
                };
                return node;
            }
        }
        if let Some(&part) = parts.get(height) {
            //在孩子节点中查找节点
            let children = self.match_children(part);
            for child in children {
                let c = child.search(parts, height + 1);
                if let Some(_) = c {
                    return c;
                }
            }
        }
        None
    }

    //根据输入的节点路径，查找节点，该方法现在只要添加中单件时使用（算法要优化）添加节点中间件时用
    pub fn search_mut(&mut self, parts: &Vec<&str>, height: usize) -> Option<&mut Node> {
        if let Some(part) = &self.part {
            if parts.len() == height || part.starts_with('*') {
                let node = match self.pattern {
                    Some(_) => Some(self),
                    None => None,
                };
                return node;
            }
        }
        if let Some(&part) = parts.get(height) {
            let children = self.match_children_mut(part);
            for child in children {
                let c = child.search_mut(parts, height + 1);
                if let Some(_) = c {
                    return c;
                }
            }
        }
        None
    }
    //路由节点提取以及，路由参数提取：代码可放在router中，呵呵
    pub fn get_node_info_by_root_node<'a>(
        root: &'a Node,
        path_parts: &Vec<&str>,
    ) -> (Option<&'a Node>, HashMap<String, String>) {
        let mut params = HashMap::new();
        let node = root.search(&path_parts, 0);
        if let Some(node) = node {
            if let Some(pattern) = &node.pattern {
                let parts = utils::parse_pattern(pattern);
                for (index, &part) in parts.iter().enumerate() {
                    if part.starts_with(':') {
                        params.insert(String::from(&part[1..]), String::from(path_parts[index]));
                    }
                    if part.starts_with('*') && parts.len() > 1 {
                        let join_vec = path_parts.get(index..).unwrap().to_vec();
                        params.insert(String::from(&part[1..]), join_vec.join("/"));
                    }
                }
                return (Some(node), params);
            }
        }
        (None, params)
    }

    pub fn get_middlewares(&self) -> Vec<&Handler> {
        let mut middlewares: Vec<&Handler> = Vec::with_capacity(self.middlewares.len());
        for handler in self.middlewares.iter() {
            middlewares.push(handler.as_ref());
        }
        middlewares
    }
    //是否精匹配
    fn is_wild(part: &str) -> bool {
        return part.starts_with(':') || part.starts_with('*');
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("pattern", &self.pattern)
            .field("part", &self.part)
            .field("children", &self.children)
            .field("is_wild", &self.is_wild)
            .finish()
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.part.eq(&other.part)
            && self.pattern.eq(&other.pattern)
            && self.is_wild.eq(&other.is_wild)
            && self.children.eq(&other.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils;

    #[test]
    fn test_tree_node_build1() {
        let mut root = Node::new();
        let pattern = "/user/index";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let target = get_target_tree1();

        debug_assert_eq!(&target, &root);

        fn get_target_tree1() -> Node {
            let mut root = Node::new();
            let mut child1 = Node::new();
            child1.part = Some("user".to_string());
            let mut child11 = Node::new();
            child11.part = Some("index".to_string());
            child11.pattern = Some("/user/index".to_string());
            child1.children.push(child11);
            root.children.push(child1);
            root
        }
    }

    #[test]
    fn test_tree_node_build2() {
        let mut root = Node::new();
        let pattern = "/hello/:name";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let target = get_target_tree2();

        debug_assert_eq!(&target, &root);

        fn get_target_tree2() -> Node {
            let mut root = Node::new();
            let mut child1 = Node::new();
            child1.part = Some("hello".to_string());
            let mut child11 = Node::new();
            child11.part = Some(":name".to_string());
            child11.pattern = Some("/hello/:name".to_string());
            child11.is_wild = true;
            child1.children.push(child11);
            root.children.push(child1);
            root
        }
    }

    #[test]
    fn test_tree_node_build3() {
        let mut root = Node::new();
        let pattern = "/static/*filepath";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let target = get_target_tree3();

        debug_assert_eq!(&target, &root);

        fn get_target_tree3() -> Node {
            let mut root = Node::new();
            let mut child1 = Node::new();
            child1.part = Some("static".to_string());
            let mut child11 = Node::new();
            child11.part = Some("*filepath".to_string());
            child11.pattern = Some("/static/*filepath".to_string());
            child11.is_wild = true;
            child1.children.push(child11);
            root.children.push(child1);
            root
        }
    }

    #[test]
    fn test_tree_node_search() {
        let mut root = Node::new();
        let pattern = "/user/index";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let path = "/user/index";
        let path_parts = utils::parse_pattern(path);
        let info = Node::get_node_info_by_root_node(&root, &path_parts);
        let mut target = Node::new();
        target.part = Some("index".to_string());
        target.pattern = Some("/user/index".to_string());
        target.is_wild = false;
        let target_params = HashMap::new();

        //结果assert
        debug_assert_eq!(Some(&target), info.0);
        debug_assert_eq!(target_params, info.1);
    }
    #[test]
    fn test_tree_node_search_param_get() {
        let mut root = Node::new();
        let pattern = "/hello/:name";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let path = "/hello/zs";
        let path_parts = utils::parse_pattern(path);
        let info = Node::get_node_info_by_root_node(&root, &path_parts);
        let mut target = Node::new();
        target.part = Some(":name".to_string());
        target.pattern = Some("/hello/:name".to_string());
        target.is_wild = true;
        let mut target_params = HashMap::new();
        target_params.insert("name".to_string(), "zs".to_string());
        
        debug_assert_eq!(Some(&target), info.0);
        debug_assert_eq!(target_params, info.1);
    }

    #[test]
    fn test_tree_node_search_param_get2() {
        let mut root = Node::new();
        let pattern = "/static/*imagefile";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let path = "/static/image1.jpg";
        let path_parts = utils::parse_pattern(path);
        let info = Node::get_node_info_by_root_node(&root, &path_parts);
        let mut target = Node::new();
        target.part = Some("*imagefile".to_string());
        target.pattern = Some("/static/*imagefile".to_string());
        target.is_wild = true;
        let mut target_params = HashMap::new();
        target_params.insert("imagefile".to_string(), "image1.jpg".to_string());


        debug_assert_eq!(Some(&target), info.0);
        debug_assert_eq!(target_params, info.1);
    }

    #[test]
    fn test_tree_node_search_param_get3() {
        let mut root = Node::new();
        let pattern = "/static/*imagefile";
        let parts = utils::parse_pattern(pattern);
        root.insert(pattern, parts, None, 0);
        let path = "/static/user/image1.jpg";
        let path_parts = utils::parse_pattern(path);
        let info = Node::get_node_info_by_root_node(&root, &path_parts);
        let mut target = Node::new();
        target.part = Some("*imagefile".to_string());
        target.pattern = Some("/static/*imagefile".to_string());
        target.is_wild = true;
        let mut target_params = HashMap::new();
        target_params.insert("imagefile".to_string(), "user/image1.jpg".to_string());

        

        debug_assert_eq!(Some(&target), info.0);
        debug_assert_eq!(target_params, info.1);
    }
}
