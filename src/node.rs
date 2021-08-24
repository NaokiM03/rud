use std::collections::LinkedList;

#[derive(Debug, Clone, PartialEq)]
enum RudStr {
    StringLit(String),
    // Expr,
}

impl RudStr {
    fn to_s(self) -> String {
        if let RudStr::StringLit(s) = self {
            s
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RudStdFn {
    Puts { s: RudStr },
}

impl RudStdFn {
    pub fn new_from_string(s: &str) -> RudStdFn {
        let rud_str = RudStr::StringLit(s.to_string());
        RudStdFn::Puts { s: rud_str }
    }
}

impl RudStdFn {
    fn to_code(self) -> String {
        match self {
            RudStdFn::Puts { s } => format!("println!(\"{}\");", s.to_s()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RudType {
    Usize,
    None,
}

impl RudType {
    pub fn try_from(s: &str) -> Option<RudType> {
        match s {
            "usize" => Some(RudType::Usize),
            _ => Some(RudType::None),
        }
    }

    pub fn to_s(self) -> Option<String> {
        match self {
            RudType::Usize => Some("usize".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RudArg {
    name: String,
    rud_type: RudType,
}

impl RudArg {
    pub fn new(name: &str, rud_type: RudType) -> RudArg {
        RudArg { name: name.to_string(), rud_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFn {
    is_public: bool,
    name: String,
    args: Vec<RudArg>,
    return_type: RudType,
    inner: Box<NodeList>,
}

impl UserDefinedFn {
    pub fn new(is_public: bool, name: &str, args: Vec<RudArg>, return_type: RudType, inner: NodeList) -> UserDefinedFn {
        UserDefinedFn {
            is_public,
            name: name.to_owned(),
            args,
            return_type,
            inner: Box::new(inner),
        }
    }
}

impl UserDefinedFn {
    fn to_code(self) -> String {
        let mut s = "".to_string();
        if self.is_public {
            s.push_str("pub ");
        }
        s.push_str("fn ");
        s.push_str(&self.name);
        s.push_str("(");
        if self.args.len() > 0 {}
        s.push_str(") ");
        if self.return_type != RudType::None {}
        s.push('{');
        s.push('\n');
        for node in self.inner.clone().into_iter() {
            s.push_str("    ");
            s.push_str(&node.to_code());
            s.push('\n');
        }
        s.push('}');
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    RudStdFn(RudStdFn),
    UserDefinedFn(UserDefinedFn),
}

impl Node {
    pub fn to_code(self) -> String {
        match self {
            Node::RudStdFn(rud_std_fn) => rud_std_fn.to_code(),
            Node::UserDefinedFn(user_defined_fn) => user_defined_fn.to_code(),
            _ => "".to_string(),
        }
    }
}

pub type NodeList = LinkedList<Node>;

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
pub mod tests;
