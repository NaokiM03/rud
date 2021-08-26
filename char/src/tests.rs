use super::*;

#[test]
fn is_alphabetic() {
    fn check(c: char) {
        assert!(Char(c).is_alphabetic());
    }

    ('a'..='z').for_each(|c| check(c));
    ('A'..='Z').for_each(|c| check(c));
}
