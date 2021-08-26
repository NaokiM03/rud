/// thin wrapper of char type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Char(char);

const ZERO: Char = Char('0');
const LINE_TERM: Char = Char('\n');
const WHITE_SPACE: Char = Char(' ');
const DOUBLE_QUOTATION: Char = Char('"');
const AMPERSAND: Char = Char('&');
const SINGLE_QUOTATION: Char = Char('\'');
const LEFT_PARENTHESIS: Char = Char('(');
const RIGHT_PARENTHESIS: Char = Char(')');
const ASSIGN: Char = Char('=');
const UNDERSCORE: Char = Char('_');

impl Char {
    pub const fn new(c: char) -> Char {
        Char(c)
    }
    pub const fn to_char(&self) -> char {
        self.0
    }
}

// compare with single letter
impl Char {
    pub const fn is_zero(&self) -> bool {
        matches!(*self, ZERO)
    }
    pub const fn is_line_term(&self) -> bool {
        matches!(*self, LINE_TERM)
    }
    pub const fn is_whitespace(&self) -> bool {
        matches!(*self, WHITE_SPACE)
    }
    pub const fn is_double_quotation(&self) -> bool {
        matches!(*self, DOUBLE_QUOTATION)
    }
    pub const fn is_anpersand(&self) -> bool {
        matches!(*self, AMPERSAND)
    }
    pub const fn is_single_quotation(&self) -> bool {
        matches!(*self, SINGLE_QUOTATION)
    }
    pub const fn is_left_parenthesis(&self) -> bool {
        matches!(*self, LEFT_PARENTHESIS)
    }
    pub const fn is_right_parenthesis(&self) -> bool {
        matches!(*self, RIGHT_PARENTHESIS)
    }
    pub const fn is_assign(&self) -> bool {
        matches!(*self, ASSIGN)
    }
    pub const fn is_underscore(&self) -> bool {
        matches!(*self, UNDERSCORE)
    }
}

// categorizing characters
impl Char {
    pub const fn is_number(&self) -> bool {
        self.0.is_ascii_digit()
    }
    pub const fn is_lowercase(&self) -> bool {
        match self.0 {
            'a'..='z' => true,
            _ => false,
        }
    }
    pub const fn is_uppercase(&self) -> bool {
        match self.0 {
            'A'..='Z' => true,
            _ => false,
        }
    }
    pub const fn is_alphabetic(&self) -> bool {
        self.is_lowercase() || self.is_uppercase()
    }
    pub const fn is_ascii_punctuation(&self) -> bool {
        self.0.is_ascii_punctuation()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
pub mod tests;
