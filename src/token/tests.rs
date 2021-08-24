use super::*;

mod token_kind {
    use super::*;

    #[test]
    fn try_get_identifier_name() {
        let token_kind = TokenKind::Identifier("Hello, rud!".to_string());
        assert_eq!(
            token_kind.try_get_identifier_name(),
            &"Hello, rud!".to_string()
        );
    }

    #[test]
    fn try_get_idnetifier_type() {
        let token_kind = TokenKind::Identifier("usize".to_string());
        assert_eq!(token_kind.try_get_idnetifier_type(), RudType::Usize);
    }

    #[test]
    fn try_get_rud_std_fn_name() {
        let token_kind = TokenKind::RudStdFn(RudStdFn::Puts);
        assert_eq!(*token_kind.try_get_rud_std_fn_name(), RudStdFn::Puts);
    }
}

mod tokens {
    use super::*;

    // #[test]
    // fn split() {

    // }

    #[test]
    fn to_rud_std_fn_node() {
        let node = create_rud_std_fn_tokens().to_rud_std_fn_node();
        let expect = Node::RudStdFn(node::RudStdFn::new_from_string("Hello, Rud!"));
        assert_eq!(node, expect);
    }

    #[test]
    fn to_user_defined_fn() {
        use crate::node::tests::*;

        let node = create_user_defined_fn_tokens().to_user_defined_fn();
        let expect = Node::UserDefinedFn(new_user_defined_fn());
        assert_eq!(node, expect);
    }
}

// -------------------------------------------------------------------------------------------------

fn dummy_pos() -> TokenPos {
    TokenPos { start: 0, end: 0 }
}

fn create_rud_std_fn_tokens() -> Tokens {
    let rud_std_fn = Token::new_rud_std_fn(RudStdFn::Puts, dummy_pos());
    let left_parenthesis = Token::new_punct(Punct::LeftParenthesis, dummy_pos());
    let string_lit = Token::new_string_lit("Hello, Rud!".to_string(), dummy_pos());
    let right_parenthesis = Token::new_punct(Punct::RightParenthesis, dummy_pos());
    let mut tokens = Tokens::new();
    tokens.push(rud_std_fn);
    tokens.push(left_parenthesis);
    tokens.push(string_lit);
    tokens.push(right_parenthesis);
    tokens
}

fn create_user_defined_fn_tokens() -> Tokens {
    let function = Token::new_reserved(Reserved::Fn, dummy_pos());
    let name = Token::new_identifier("main".to_string(), dummy_pos());
    let left_parenthesis = Token::new_punct(Punct::LeftParenthesis, dummy_pos());
    let right_parenthesis = Token::new_punct(Punct::RightParenthesis, dummy_pos());
    let assign = Token::new_punct(Punct::Assign, dummy_pos());
    let line_term = Token::new_line_term(dummy_pos());
    let indent = Token::new_indent(dummy_pos());
    let mut tokens = Tokens::new();
    tokens.push(function);
    tokens.push(name);
    tokens.push(left_parenthesis);
    tokens.push(right_parenthesis);
    tokens.push(assign);
    tokens.push(line_term);
    tokens.push(indent);
    let mut rud_std_fn_tokens = create_rud_std_fn_tokens();
    tokens.append(&mut rud_std_fn_tokens);
    tokens
}

pub fn create_rud_std_fn_node() -> Node {
    create_rud_std_fn_tokens().to_rud_std_fn_node()
}

pub fn create_user_defined_fn_node() -> Node {
    create_user_defined_fn_tokens().to_user_defined_fn()
}
