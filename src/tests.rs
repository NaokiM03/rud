use crate::{
    node::CodeGenerator,
    source::{Lexer, Source},
    token::Parse,
};

#[test]
fn source2code() {
    let rud_code = r#"fn main() =
    puts("Hello, Rud!")
"#;

    let expect = r#"fn main() {
    println!("Hello, Rud!");
}"#;

    let mut source = Source::new(rud_code);
    let tokens = source.to_tokens();
    let rust_code = tokens.to_user_defined_fn().to_code();

    assert_eq!(rust_code, expect);
}
