#![feature(test)]
#![feature(ascii_ctype)]

extern crate test;

use std::ascii::AsciiExt;

pub fn valid_scope(scope: &str) -> bool {
    scope.chars().all(|ch| ch.is_ascii_graphic() || ch == ' ')
}

#[cfg(test)]
mod valid_scope_tests {
    use super::*;
    use test::Bencher;

    macro_rules! test_and_bench {
        ($n:ident, $m:ident, $thing:expr, $res:expr) => {
            #[test]
            fn $n() {
                assert_eq!($res, valid_scope($thing));
            }
            #[bench]
            fn $m(b: &mut Bencher) {
                b.iter(|| valid_scope($thing));
            }
        }
    }
    test_and_bench!(normal_scopes, normal_scopes_bench, "auth:credentials", true);
    test_and_bench!(empty_scopes, empty_scopes_bench, "", true);
    test_and_bench!(start_scopes, star_scopes_bench, "queue:*", true);
    test_and_bench!(scopes_with_spaces, scopes_with_spaces_bench, "secrets:garbage:foo bar", true);
    test_and_bench!(scopes_with_newlines, scopes_with_newlines_bench, "some:garbage\nauth:credentials", false);
    test_and_bench!(scopes_with_nulls, scopes_with_nulls_bench, "some:garbage\0auth:credentials", false);
    test_and_bench!(scopes_with_unicode, scopes_with_unicode_bench, "halt:👻", false);
}
