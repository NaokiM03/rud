use super::*;

mod source {
    use super::*;

    #[test]
    fn is_indent() {
        let source = Source::new("    foo");
        assert!(source.is_indent());
    }

    #[test]
    fn peek_string() {
        let source = Source::new("foo_bar_123\nfoobar aaa");
        assert_eq!(source.peek_string(), "foo_bar_123\nfoobar");
    }

    mod lexer {
        use super::*;

        mod skip_whitespace {
            use super::*;

            #[test]
            fn one_whitespace() {
                let mut source = Source::new(" ");
                let pre_pos = source.cursor.pos;
                source.skip_whitespace();
                let post_pos = source.cursor.pos;
                assert_eq!(pre_pos + 1, post_pos);
            }

            #[test]
            fn some_whitespace() {
                let mut source = Source::new("   ");
                let pre_pos = source.cursor.pos;
                while source.is_end().not() {
                    source.skip_whitespace();
                }
                let post_pos = source.cursor.pos;
                assert_eq!(pre_pos + 3, post_pos);
            }

            #[test]
            fn more_than_4() {
                let mut source = Source::new("    ");
                let pre_pos = source.cursor.pos;
                source.skip_whitespace();
                let post_pos = source.cursor.pos;
                assert_eq!(pre_pos, post_pos);
            }

            #[test]
            fn line_term() {
                let mut source = Source::new("\n");
                let pre_pos = source.cursor.pos;
                source.skip_whitespace();
                let post_pos = source.cursor.pos;
                assert_eq!(pre_pos, post_pos);
            }
        }

        #[test]
        fn try_get_indent_token() {
            let mut ok = Source::new("    ");
            assert!(ok.try_get_indent_token().is_some());
            assert_eq!(ok.cursor.pos, 4);

            let mut ng = Source::new("   ");
            assert!(ng.try_get_indent_token().is_none());
            assert_eq!(ng.cursor.pos, 0);
        }

        #[test]
        fn try_get_reserved_token() {
            let mut ok = Source::new("fn");
            assert!(ok.try_get_reserved_token().is_some());

            let mut ng = Source::new("foo");
            assert!(ng.try_get_reserved_token().is_none());
        }

        #[test]
        fn try_get_rud_std_fn_token() {
            let mut ok = Source::new("puts");
            assert!(ok.try_get_rud_std_fn_token().is_some());

            let mut ng = Source::new("foo");
            assert!(ng.try_get_rud_std_fn_token().is_none());
        }

        #[test]
        fn try_get_identifier_token() {
            let mut ok = Source::new("foo");
            assert!(ok.try_get_identifier_token().is_some());

            let mut ng = Source::new("?foo");
            assert!(ng.try_get_identifier_token().is_none());
        }

        #[test]
        fn try_get_string_lit_token() {
            let mut ok = Source::new(r#""foo""#);
            assert!(ok.try_get_string_lit_token().is_some());

            let mut ng = Source::new("foo");
            assert!(ng.try_get_string_lit_token().is_none());
        }

        #[test]
        fn try_get_punct_token() {
            fn check(c: char) {
                let mut ok = Source::new(&c.to_string());
                assert!(ok.try_get_punct_token().is_some());
            }
            let punct = ['(', ')', '='];
            punct.iter().for_each(|c| {
                check(*c);
            });
        }

        #[test]
        fn to_tokens() {
            let code = r#"fn main() =
    puts("Hello, world")
"#;
            let mut source = Source::new(code);
            source.print();
            let a = source.to_tokens();
            dbg!(&a);
        }
    }
}

// -------------------------------------------------------------------------------------------------

trait Print {
    fn print(&mut self);
}

impl Print for Source {
    fn print(&mut self) {
        let code: String = self.code.clone().into_iter().map(|x| x.to_char()).collect();
        let mut pos = 0;
        for s in code.split('\n') {
            for _ in 0..(s.len() + 1) {
                print!("{0: >03} ", pos);
                pos += 1;
            }
            print!("\n");
            for i in 0..(s.len()) {
                print!("  {} ", s.chars().nth(i).unwrap().to_string());
            }
            print!(" \\n");
            print!("\n\n");
        }
    }
}
