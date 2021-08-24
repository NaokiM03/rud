use super::*;

mod rud_str {
    use super::*;

    #[test]
    fn to_s() {
        let hello_world = "Hello, Rud!".to_string();
        let rud_str = RudStr::StringLit(hello_world.clone());
        assert_eq!(rud_str.to_s(), hello_world);
    }
}

mod rud_std_fn {
    use super::*;

    #[test]
    fn new_from_string() {
        // Testing of this function is secured by to_code().
        assert!(true);
    }

    #[test]
    fn to_code() {
        let rud_std_fn = RudStdFn::new_from_string("Hello, Rud!");
        assert_eq!(rud_std_fn.to_code(), r#"println!("Hello, Rud!");"#);
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
        let arg = RudArg::new(&name, rud_type.clone());
        let expect = RudArg { name, rud_type };
        assert_eq!(arg, expect);
    }
}

mod user_defined_fn {
    use crate::token::tests::create_user_defined_fn_node;

    #[test]
    fn new() {
        // Testing of this function is secured by to_code().
        assert!(true);
    }

    #[test]
    fn to_code() {
        let user_defined_fn = create_user_defined_fn_node();
        let expect = r#"fn main() {
    println!("Hello, Rud!");
}"#;
        assert_eq!(user_defined_fn.to_code(), expect);
    }
}

mod node {
    use crate::token::tests::*;

    mod rud_std_fn_node {
        use super::*;

        #[test]
        fn to_code() {
            let node = create_rud_std_fn_node();
            let code = node.to_code();
            let expect = r#"println!("Hello, Rud!");"#;
            assert_eq!(code, expect);
        }
    }

    mod user_defined_fn {
        use super::*;

        #[test]
        fn to_code() {
            let node = create_user_defined_fn_node();
            let code = node.to_code();
            let expect = r#"fn main() {
    println!("Hello, Rud!");
}"#;
            assert_eq!(code, expect);
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub fn new_user_defined_fn() -> UserDefinedFn {
    let hello_world = "Hello, Rud!".to_string();
    let rud_str = RudStr::StringLit(hello_world.clone());
    let rud_std_fn = RudStdFn::Puts { s: rud_str };
    let node_kind = Node::RudStdFn(rud_std_fn);
    let mut node_list = NodeList::new();
    node_list.push_back(node_kind);
    UserDefinedFn::new(false, "main", Vec::new(), RudType::None, node_list)
}
