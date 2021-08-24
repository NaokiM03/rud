use super::*;

mod rud_str {
    use super::*;

    #[test]
    fn to_s() {
        let hello_world = "Hello, world!".to_string();
        let rud_str = RudStr::StringLit(hello_world.clone());
        assert_eq!(rud_str.to_s(), hello_world);
    }
}

mod rud_std_fn {
    use super::*;

    #[test]
    fn to_code() {
        let hello_world = "Hello, world!".to_string();
        let rud_str = RudStr::StringLit(hello_world.clone());
        let rud_std_fn = RudStdFn::Puts { s: rud_str };
        assert_eq!(rud_std_fn.to_code(), r#"println!("Hello, world!");"#);
    }
}

mod rud_type {
    use super::*;

    #[test]
    fn try_from() {
        assert_eq!(RudType::try_from("usize").unwrap(), RudType::Usize);
    }

    #[test]
    fn to_s() {
        let usize = RudType::Usize;
        assert_eq!(usize.to_s().unwrap(), "usize");
    }
}

mod rud_arg {
    use super::*;

    #[test]
    fn new() {
        let name = "n".to_string();
        let rud_type = RudType::Usize;
        let arg = RudArg::new(name.clone(), rud_type.clone());
        let expect = RudArg { name, rud_type };
        assert_eq!(arg, expect);
    }
}

mod user_defined_fn {
    use super::*;

    #[test]
    fn to_code() {
        let hello_world = "Hello, Rud!".to_string();
        let rud_str = RudStr::StringLit(hello_world.clone());
        let rud_std_fn = RudStdFn::Puts { s: rud_str };
        let node_kind = Node::RudStdFn(rud_std_fn);
        let mut node_list = NodeList::new();
        node_list.push_back(node_kind);
        let user_defined_fn = UserDefinedFn {
            is_public: false,
            name: "main".to_string(),
            args: Vec::new(),
            return_type: RudType::None,
            inner: Box::new(node_list),
        };
        let expect = r#"fn main() {
    println!("Hello, Rud!");
}"#;
        assert_eq!(user_defined_fn.to_code(), expect);
    }
}

mod node {
    use super::*;

    #[test]
    fn to_code() {
    }
}
