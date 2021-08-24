use std::ops::Not;

use crate::node::{self, Node, NodeList, RudArg, RudType, UserDefinedFn};

#[derive(Debug, Clone, PartialEq)]
pub enum Punct {
    LeftParenthesis,  // (
    RightParenthesis, // )
    Colon,            // :
    Comma,            // ,
    Assign,           // =
    Plus,             // +
    Minus,            // -
    Mul,              // *
    Div,              // /
    Gt,               // >
    Lt,               // <
    Gteq,             // >=
    Lteq,             // <=
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reserved {
    Pub,
    Fn,
}

impl Reserved {
    pub fn is_reserved_word(s: &str) -> bool {
        let keywords = ["pub", "fn"];
        keywords.contains(&s)
    }

    pub fn from(s: &str) -> Reserved {
        match s {
            "pub" => Reserved::Pub,
            "fn" => Reserved::Fn,
            _ => panic!("{} is not reserved word.", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RudStdFn {
    Puts,
}

impl RudStdFn {
    pub fn is_rud_std_fn(s: &str) -> bool {
        let keywords = ["puts"];
        keywords.contains(&s)
    }

    pub fn from(s: &str) -> RudStdFn {
        match s {
            "puts" => RudStdFn::Puts,
            _ => panic!("{} is not rud standard function.", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Indent,
    Reserved(Reserved),
    Punct(Punct),
    Identifier(String),
    StringLit(String),
    RudStdFn(RudStdFn),
    LineTerm,
}

impl TokenKind {
    fn is_indent(&self) -> bool {
        *self == TokenKind::Indent
    }
    fn is_public(&self) -> bool {
        *self == TokenKind::Reserved(Reserved::Pub)
    }
    fn is_fn(&self) -> bool {
        *self == TokenKind::Reserved(Reserved::Fn)
    }
    fn is_right_parenthesis(&self) -> bool {
        *self == TokenKind::Punct(Punct::RightParenthesis)
    }
    fn is_comma(&self) -> bool {
        *self == TokenKind::Punct(Punct::Comma)
    }
    fn is_colon(&self) -> bool {
        *self == TokenKind::Punct(Punct::Colon)
    }
    fn is_identifier(&self) -> bool {
        if let TokenKind::Identifier(_) = *self {
            true
        } else {
            false
        }
    }
    fn is_string_lit(&self) -> bool {
        if let TokenKind::StringLit(_) = *self {
            true
        } else {
            false
        }
    }
    fn is_rud_std_fn(&self) -> bool {
        if let TokenKind::RudStdFn(_) = *self {
            true
        } else {
            false
        }
    }
    fn is_line_term(&self) -> bool {
        *self == TokenKind::LineTerm
    }
}

impl TokenKind {
    fn try_get_identifier_name(&self) -> &String {
        if let TokenKind::Identifier(name) = self {
            name
        } else {
            panic!("expect identifier name.")
        }
    }
    fn try_get_idnetifier_type(&self) -> RudType {
        if let TokenKind::Identifier(rud_type) = self {
            RudType::try_from(rud_type).unwrap()
        } else {
            panic!("expect identifier type.")
        }
    }
    fn try_get_rud_std_fn_name(&self) -> &RudStdFn {
        if let TokenKind::RudStdFn(rud_std_fn) = self {
            rud_std_fn
        } else {
            panic!("expect rud std fn.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenPos {
    start: usize,
    end: usize,
}

impl TokenPos {
    pub fn new(start: usize, end: usize) -> TokenPos {
        TokenPos { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    pos: TokenPos,
}

impl Token {
    #[inline]
    fn new(kind: TokenKind, pos: TokenPos) -> Token {
        Token { kind, pos }
    }
}

impl Token {
    pub fn new_indent(pos: TokenPos) -> Token {
        Token::new(TokenKind::Indent, pos)
    }

    pub fn new_reserved(reserved: Reserved, pos: TokenPos) -> Token {
        Token::new(TokenKind::Reserved(reserved), pos)
    }

    pub fn new_punct(punct: Punct, pos: TokenPos) -> Token {
        Token::new(TokenKind::Punct(punct), pos)
    }

    pub fn new_identifier(identifier: String, pos: TokenPos) -> Token {
        Token::new(TokenKind::Identifier(identifier), pos)
    }

    pub fn new_string_lit(string_lit: String, pos: TokenPos) -> Token {
        Token::new(TokenKind::StringLit(string_lit), pos)
    }

    pub fn new_rud_std_fn(rud_std_fn: RudStdFn, pos: TokenPos) -> Token {
        Token::new(TokenKind::RudStdFn(rud_std_fn), pos)
    }

    pub fn new_line_term(pos: TokenPos) -> Token {
        Token::new(TokenKind::LineTerm, pos)
    }
}

pub type Tokens = Vec<Token>;
type SplitedTokens = Vec<Tokens>;

pub trait Parse {
    fn split(self) -> SplitedTokens;

    fn to_rud_std_fn_node(self) -> Node;
    fn to_user_defined_fn(self) -> Node;
}

impl Parse for Tokens {
    // リバースしていい感じにできそう
    fn split(self) -> SplitedTokens {
        let mut last_type_is_line_term = false;
        let mut splited_tokens = SplitedTokens::new();
        let mut scrap = Tokens::new();
        for token in self {
            if last_type_is_line_term
                && token.kind.is_indent().not()
                && token.kind.is_line_term().not()
            {
                splited_tokens.push(scrap);
                scrap = Tokens::new();
            }
            last_type_is_line_term = token.kind.is_line_term();
            scrap.push(token);
        }
        if scrap.is_empty().not() {
            splited_tokens.push(scrap);
        }
        splited_tokens
    }

    fn to_rud_std_fn_node(self) -> Node {
        let mut pos = 0;

        let rus_std_fn = self[pos].kind.try_get_rud_std_fn_name();
        pos += 1; // RudStdFn
        pos += 1; // LeftParenthesis
        let string_lit = if let TokenKind::StringLit(s) = &self[pos].kind {
            s
        } else {
            ""
        };
        pos += 1; // StringLit
        pos += 1; // RightParenthesis
        assert_eq!(self.len(), pos);
        match rus_std_fn {
            RudStdFn::Puts => {
                let rsf = node::RudStdFn::new_from_string(string_lit);
                Node::RudStdFn(rsf)
            }
        }
    }

    fn to_user_defined_fn(self) -> Node {
        let mut pos = 0;

        let function_is_public = self[pos].kind.is_public();
        if function_is_public {
            pos += 1;
        }
        pos += 1; // fn
        let function_name = self[pos].kind.try_get_identifier_name();
        pos += 1; // main
        pos += 1; // (
        let mut args = Vec::new();
        while self.len() > pos && self[pos].kind.is_right_parenthesis().not() {
            let arg_name = self[pos].kind.try_get_identifier_name();
            pos += 1;
            let arg_type = self[pos].kind.try_get_idnetifier_type();
            pos += 1;
            if self[pos].kind.is_comma() {
                pos += 1;
            }
            let arg = RudArg::new(arg_name, arg_type);
            args.push(arg);
        }
        if self.len() > pos && self[pos].kind.is_right_parenthesis() {
            pos += 1; // )
        } else {
            panic!("expect right parenthesis.")
        }
        let mut return_type = RudType::None;
        if self[pos].kind.is_colon() {
            pos += 1;
            return_type = self[pos].kind.try_get_idnetifier_type();
        }
        pos += 1;
        // not 1 line function
        if self[pos].kind.is_line_term() {
            pos += 1;
            if self[pos].kind.is_indent() {
                pos += 1;
            } else {
                panic!("bad token. must be indent token after assign and line-term.")
            }
        }

        let mut node_list = NodeList::new();
        if self[pos].kind.is_rud_std_fn() {
            let mut tokens = Tokens::new();
            tokens.push(self[pos].clone());
            pos += 1;
            tokens.push(self[pos].clone());
            pos += 1;
            while self.len() > pos && self[pos].kind.is_right_parenthesis().not() {
                tokens.push(self[pos].clone());
                pos += 1;
            }
            tokens.push(self[pos].clone());
            pos += 1;
            if self.len() > pos && self[pos].kind.is_line_term() {
                pos += 1;
            }
            node_list.push_back(tokens.to_rud_std_fn_node());
        }

        let user_defined_fn = UserDefinedFn::new(
            function_is_public,
            function_name,
            args,
            return_type,
            node_list,
        );
        Node::UserDefinedFn(user_defined_fn)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
pub mod tests;
