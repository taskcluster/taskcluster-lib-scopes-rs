#![feature(test)]
#![feature(ascii_ctype)]

extern crate test;

use std::ascii::AsciiExt;

pub fn valid_scope(scope: &str) -> bool {
    scope.chars().all(|ch| ch.is_ascii_graphic() || ch == ' ')
}

pub fn scope_match(scope_patterns: &[&str], scopesets: &[&[&str]]) -> bool {
    scopesets.iter().any(|scopeset| {
        scopeset.iter().all(|scope| {
            scope_patterns.iter().any(|pattern| {
                match pattern.chars().last() {
                    Some('*') => scope.starts_with(pattern.trim_right_matches('*')),
                    _ => scope == pattern,
                }
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    mod valid_scope {
        use super::*;
        macro_rules! test_valid_scope {
            ($n:ident, $thing:expr, $res:expr) => {
                mod $n {
                    use super::*;
                    #[test]
                    fn correctness() {
                        assert_eq!($res, valid_scope($thing));
                    }
                    #[bench]
                    fn benchmark(b: &mut Bencher) {
                        b.iter(|| valid_scope($thing));
                    }
                }
            }
        }
        test_valid_scope!(normal_scopes, "auth:credentials", true);
        test_valid_scope!(empty_scopes, "", true);
        test_valid_scope!(star_scopes, "queue:*", true);
        test_valid_scope!(scopes_with_spaces, "secrets:garbage:foo bar", true);
        test_valid_scope!(scopes_with_newlines, "some:garbage\nauth:credentials", false);
        test_valid_scope!(scopes_with_nulls, "some:garbage\0auth:credentials", false);
        test_valid_scope!(scopes_with_unicode, "halt:👻", false);
    }

    mod scope_match {
        use super::*;
        macro_rules! test_scope_match {
            ($n:ident, $patterns:expr, $scopes:expr, $res:expr) => {
                mod $n {
                    use super::*;
                    #[test]
                    fn correctness() {
                        assert_eq!($res, scope_match($patterns, $scopes));
                    }
                    #[bench]
                    fn benchmark(b: &mut Bencher) {
                        b.iter(|| scope_match($patterns, $scopes));
                    }
                }
            }
        }
        test_scope_match!(single_exact_match, &["foo:bar"], &[&["foo:bar"]], true);
        test_scope_match!(empty_scopeset, &["foo:bar"], &[&[""]], false);
        test_scope_match!(prefix, &["foo:*"], &[&["foo:bar"]], true);
        test_scope_match!(star_in_middle, &["foo:*:bing"], &[&["foo:bar:bing"]], false);
        test_scope_match!(star_at_beginning, &["*:bar"], &[&["foo:bar"]], false);
        test_scope_match!(no_star_prefix, &["foo:"], &[&["foo:bar"]], false);
        test_scope_match!(star_but_not_prefix, &["foo:bar:*"], &[&["bar:bing"]], false);
        test_scope_match!(star_but_not_prefix_partial, &["bar:*"], &[&["foo:bar:bing"]], false);
        test_scope_match!(disjunction_strings, &["bar:*"], &[&["foo:x"], &["bar:x"]], true);
        test_scope_match!(conjunction, &["bar:*", "foo:x"], &[&["foo:x", "bar:y"]], true);
        test_scope_match!(empty_pattern, &[""], &[&["foo:bar"]], false);
        test_scope_match!(empty_patterns, &[], &[&["foo:bar"]], false);
        test_scope_match!(bare_star, &["*"], &[&["foo:bar", "bar:bing"]], true);
        test_scope_match!(empty_scopeset_conjunction, &["foo:bar"], &[&[]], true);
        test_scope_match!(empty_scopeset_disjunction, &["foo:bar"], &[], false);
    }
}
