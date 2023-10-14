pub(crate) fn tag<'a>(prefix: &'a str, s: &'a str) -> &'a str {
    let s = s.strip_prefix(prefix).expect("Expected: {prefix}");
    s
}

pub(crate) fn extract_digit(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit() || c == '-', s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    take_while(|c| matches!(c, '+' | '-' | '*' | '/'), s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_whitespace(), s)
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    if s.starts_with(|c: char| c.is_numeric()) {
        return (s, "");
    }

    let pred = |c: char| -> bool { c.is_ascii_alphanumeric() || c == '_' };
    take_while(pred, s)
}

pub(crate) fn take_while(pred: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let char_end = s
        .char_indices()
        .find_map(|(idx, c)| if pred(c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..char_end];
    let remainder = &s[char_end..];

    (remainder, extracted)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_tag() {
        assert_eq!(tag("let", "let a = b"), " a = b");
        assert_eq!(tag("=", "= b"), " b");
    }

    #[test]
    fn extract_ident_alpha() {
        assert_eq!(extract_ident("abcd aaaa"), (" aaaa", "abcd"));
        assert_eq!(extract_ident("foo()"), ("()", "foo"));
        assert_eq!(extract_ident("hello_world()"), ("()", "hello_world"));
    }

    #[test]
    fn extract_ident_alphanumeric() {
        assert_eq!(extract_ident("a2"), ("", "a2"));
        assert_eq!(extract_ident("bar123()"), ("()", "bar123"));
        assert_eq!(extract_ident("baz_999()"), ("()", "baz_999"));
    }
    #[test]
    fn cannot_extract_ident_starting_with_number() {
        assert_eq!(extract_ident("123abc"), ("123abc", ""));
        assert_eq!(extract_ident("1000"), ("1000", ""));
    }

    #[test]
    fn extract_whitespace_basic() {
        assert_eq!(extract_whitespace("  \t   1"), ("1", "  \t   "));
        assert_eq!(extract_whitespace("     200"), ("200", "     "));
        assert_eq!(extract_whitespace(" 200\r\n"), ("200\r\n", " "));
        assert_eq!(
            extract_whitespace("\r\nHello World\r\n"),
            ("Hello World\r\n", "\r\n")
        );
    }

    #[test]
    fn extract_empty() {
        assert_eq!(extract_digit(""), ("", ""));
    }

    #[test]
    fn extract_digit_big() {
        assert_eq!(extract_digit("123456+99999"), ("+99999", "123456"));
        assert_eq!(extract_digit("1234aaaa"), ("aaaa", "1234")); // Not sure if correct
    }

    #[test]
    fn extract_digit_negative() {
        assert_eq!(extract_digit("-300+1"), ("+1", "-300"));
        assert_eq!(extract_digit("-20"), ("", "-20"));
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digit("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_op_basic() {
        assert_eq!(extract_op("+20"), ("20", "+"));
        assert_eq!(extract_op("-20"), ("20", "-"));
        assert_eq!(extract_op("-100"), ("100", "-"));
        assert_eq!(extract_op("*20"), ("20", "*"));
        assert_eq!(extract_op("*4"), ("4", "*"));
        assert_eq!(extract_op("/20"), ("20", "/"));
        assert_eq!(extract_op("/33"), ("33", "/"));
        assert_eq!(extract_op("20"), ("20", ""));
        assert_eq!(extract_op(""), ("", ""));
    }
}
