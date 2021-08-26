use std::ops::Not;

use crate::token::{Punct, Reserved, RudStdFn, Token, TokenPos, Tokens};

use rud_char::Char;

// #[derive(Debug, Clone)]
// pub struct Line {
//     number: usize,
//     start: usize,
//     end: usize,
// }

// impl Line {
//     fn new(number: usize, start: usize, end: usize) -> Line {
//         Line { number, start, end }
//     }
// }

#[derive(Debug, Clone)]
struct Cursor {
    pos: usize,
    tmp: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { pos: 0, tmp: 0 }
    }
}

impl Cursor {
    fn next(&mut self) {
        self.pos += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    code: Vec<Char>,
    len: usize,
    cursor: Cursor,
    // lines: Vec<Line>,
    // line: Line,
}

impl Source {
    pub fn new(source_code: &str) -> Source {
        let code = source_code
            .chars()
            .map(|x| Char::new(x))
            .collect::<Vec<Char>>();
        let len = code.len();
        let cursor = Cursor::new();
        Source { code, len, cursor }
    }
}

impl Source {
    fn is_end(&self) -> bool {
        self.cursor.pos >= self.len
    }

    fn get_char(&self, pos: usize) -> Char {
        self.code[pos]
    }

    fn peek(&self) -> Char {
        if self.is_end() {
            panic!("cursor is end of file.")
        }
        self.get_char(self.cursor.pos)
    }

    fn peek_string(&self) -> String {
        let mut pos = self.cursor.pos;
        let mut s = "".to_string();
        while pos < self.len {
            match self.get_char(pos) {
                c if c.is_alphabetic() || c.is_number() || c.is_underscore() || c.is_line_term() => {
                    s.push(c.to_char());
                }
                c if c.is_whitespace() => {
                    break;
                }
                c if c.is_ascii_punctuation() => {
                    break;
                }
                _ => {
                    dbg!(self.peek());
                    panic!();
                }
            }
            pos += 1;
        }
        s
    }

    fn is_whitespace(&self) -> bool {
        self.peek().is_whitespace()
    }

    fn is_indent(&self) -> bool {
        let pos = self.cursor.pos;
        if self.len - pos < 4 {
            return false;
        }
        self.get_char(pos).is_whitespace()
            && self.get_char(pos + 1).is_whitespace()
            && self.get_char(pos + 2).is_whitespace()
            && self.get_char(pos + 3).is_whitespace()
    }

    fn is_line_term(&self) -> bool {
        self.peek().is_line_term()
    }

    fn is_punct(&self) -> bool {
        self.peek().is_ascii_punctuation()
    }
}

pub trait Lexer {
    fn skip_whitespace(&mut self);
    fn try_get_line_term_token(&mut self) -> Option<Token>;
    fn try_get_indent_token(&mut self) -> Option<Token>;
    fn try_get_reserved_token(&mut self) -> Option<Token>;
    fn try_get_rud_std_fn_token(&mut self) -> Option<Token>;
    fn try_get_identifier_token(&mut self) -> Option<Token>;
    fn try_get_string_lit_token(&mut self) -> Option<Token>;
    fn try_get_punct_token(&mut self) -> Option<Token>;
    fn to_tokens(&mut self) -> Tokens;
}

impl Lexer for Source {
    fn skip_whitespace(&mut self) {
        if self.is_whitespace() && self.is_indent().not() && self.is_line_term().not() {
            self.cursor.next();
        }
    }

    fn try_get_line_term_token(&mut self) -> Option<Token> {
        if self.is_line_term() {
            let pos = TokenPos::new(self.cursor.pos, self.cursor.pos);
            self.cursor.next();
            Some(Token::new_line_term(pos))
        } else {
            None
        }
    }

    fn try_get_indent_token(&mut self) -> Option<Token> {
        if self.is_indent() {
            let start = self.cursor.pos;
            self.cursor.pos += 4;
            let end = self.cursor.pos - 1;
            Some(Token::new_indent(TokenPos::new(start, end)))
        } else {
            None
        }
    }

    fn try_get_reserved_token(&mut self) -> Option<Token> {
        if self.is_line_term() {
            return None;
        }

        let s = self.peek_string();
        if Reserved::is_reserved_word(&s) {
            let start = self.cursor.pos;
            self.cursor.pos += s.len();
            let end = self.cursor.pos - 1;
            Some(Token::new_reserved(
                Reserved::from(&s),
                TokenPos::new(start, end),
            ))
        } else {
            None
        }
    }

    fn try_get_rud_std_fn_token(&mut self) -> Option<Token> {
        if self.is_line_term() {
            return None;
        }

        let s = self.peek_string();
        if RudStdFn::is_rud_std_fn(&s) {
            let start = self.cursor.pos;
            self.cursor.pos += s.len();
            let end = self.cursor.pos - 1;
            Some(Token::new_rud_std_fn(
                RudStdFn::from(&s),
                TokenPos::new(start, end),
            ))
        } else {
            None
        }
    }

    fn try_get_identifier_token(&mut self) -> Option<Token> {
        if self.is_line_term() {
            return None;
        }

        let s = self.peek_string();
        if s.is_empty().not() {
            let start = self.cursor.pos;
            self.cursor.pos += s.len();
            let end = self.cursor.pos - 1;
            Some(Token::new_identifier(s, TokenPos::new(start, end)))
        } else {
            None
        }
    }

    fn try_get_string_lit_token(&mut self) -> Option<Token> {
        if self.peek().is_double_quotation() {
            let start = self.cursor.pos;
            self.cursor.next();
            let mut s = "".to_string();
            while self.peek().is_double_quotation().not() {
                s.push(self.peek().to_char());
                self.cursor.next();
            }
            self.cursor.next();
            let end = self.cursor.pos - 1;
            Some(Token::new_string_lit(s, TokenPos::new(start, end)))
        } else {
            None
        }
    }

    fn try_get_punct_token(&mut self) -> Option<Token> {
        if self.is_punct().not() {
            return None;
        }
        let pos = TokenPos::new(self.cursor.pos, self.cursor.pos);
        let token = match self.peek() {
            c if c.is_left_parenthesis() => Some(Token::new_punct(Punct::LeftParenthesis, pos)),
            c if c.is_right_parenthesis() => Some(Token::new_punct(Punct::RightParenthesis, pos)),
            c if c.is_assign() => Some(Token::new_punct(Punct::Assign, pos)),
            _ => None,
        };
        self.cursor.next();
        token
    }

    fn to_tokens(&mut self) -> Tokens {
        let mut tokens = Tokens::new();

        while self.is_end().not() {
            self.skip_whitespace();
            if let Some(token) = self.try_get_line_term_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_indent_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_reserved_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_rud_std_fn_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_identifier_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_string_lit_token() {
                tokens.push(token);
                continue;
            }
            if let Some(token) = self.try_get_punct_token() {
                tokens.push(token);
                continue;
            }
            panic!("");
        }
        tokens
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
pub mod tests;
