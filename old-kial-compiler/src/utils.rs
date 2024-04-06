pub(crate) fn tag<'a>(prefix: &'a str, s: &'a str) -> Result<&'a str, String> {
    match s.strip_prefix(prefix) {
        Some(s) => Ok(s),
        None => Err(format!("Expected: {prefix}")),
    }
}

pub(crate) fn extract_digit(s: &str) -> Result<(&str, &str), String> {
    let is_negative_number = |idx, c| idx == 0 && c == '-' && s.len() > 1;
    let is_positive_number = |idx, c| idx == 0 && c == '+' && s.len() > 1;

    take_while1(
        |idx, c| c.is_ascii_digit() || is_negative_number(idx, c) || is_positive_number(idx, c),
        s,
        String::from("Expected: digits"),
    )
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    // FIXME This can just be: let s = s.strip();
    take_while(|_, c| c.is_ascii_whitespace(), s)
}

pub(crate) fn extract_string(s: &str) -> (&str, &str) {
    take_while(|_, c| c != '"', s)
}

pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |_, c| c.is_ascii_whitespace(),
        s,
        String::from("Expected: whitespace"),
    )
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let s = s.trim();

    if s.starts_with(|c: char| c.is_numeric()) {
        return Err(String::from("Expected: identifier"));
    }

    let reserved_keywords = ["let", "func"];
    for keyword in reserved_keywords {
        if s.starts_with(keyword) {
            return Err(format!("Error: '{keyword}' is a reserved keyword"));
        }
    }

    let pred = |_: usize, c: char| -> bool { c.is_ascii_alphanumeric() || c == '_' };
    take_while1(pred, s, String::from("Expected: identifier"))
}

pub(crate) fn take_while(pred: impl Fn(usize, char) -> bool, s: &str) -> (&str, &str) {
    let char_end = s
        .char_indices()
        .find_map(|(idx, c)| if pred(idx, c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..char_end];
    let remainder = &s[char_end..];

    (remainder, extracted)
}

pub(crate) fn take_while1(
    pred: impl Fn(usize, char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(pred, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_ident_expected_errors() {
        assert_eq!(
            extract_ident("func"),
            Err("Error: 'func' is a reserved keyword".to_string())
        );

        assert_eq!(
            extract_ident("let"),
            Err("Error: 'let' is a reserved keyword".to_string())
        );
    }

    #[test]
    fn remove_tag() {
        assert_eq!(tag("let", "let a = b"), Ok(" a = b"));
        assert_eq!(tag("=", "= b"), Ok(" b"));
    }

    #[test]
    fn extract_ident_alpha() {
        assert_eq!(extract_ident("abcd aaaa"), Ok((" aaaa", "abcd")));
        assert_eq!(extract_ident("foo()"), Ok(("()", "foo")));
        assert_eq!(extract_ident("hello_world()"), Ok(("()", "hello_world")));
    }

    #[test]
    fn extract_ident_alphanumeric() {
        assert_eq!(extract_ident("a2"), Ok(("", "a2")));
        assert_eq!(extract_ident("bar123()"), Ok(("()", "bar123")));
        assert_eq!(extract_ident("baz_999()"), Ok(("()", "baz_999")));
    }
    #[test]
    fn cannot_extract_ident_starting_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err(String::from("Expected: identifier"))
        );

        assert_eq!(
            extract_ident("1000"),
            Err(String::from("Expected: identifier"))
        );
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
    fn do_not_extract_whitespace_when_it_is_required() {
        assert_eq!(
            extract_whitespace1("bar"),
            Err(String::from("Expected: whitespace"))
        );
    }

    #[test]
    fn do_not_extract_extract_empty_digits() {
        assert_eq!(extract_digit(""), Err(String::from("Expected: digits")));
    }

    #[test]
    fn extract_digit_big() {
        assert_eq!(extract_digit("123456+99999"), Ok(("+99999", "123456")));
        assert_eq!(extract_digit("1234aaaa"), Ok(("aaaa", "1234"))); // Not sure if correct
    }

    #[test]
    fn extract_digit_negative() {
        assert_eq!(extract_digit("-300+1"), Ok(("+1", "-300")));
        assert_eq!(extract_digit("-20"), Ok(("", "-20")));
    }

    #[test]
    fn extract_digit_positive() {
        assert_eq!(extract_digit("+20"), Ok(("", "+20")));
    }

    #[test]
    fn extract_digit_bad_input() {
        assert_eq!(extract_digit("-"), Err("Expected: digits".to_string()));
        assert_eq!(extract_digit("+"), Err("Expected: digits".to_string()));
        assert_eq!(extract_digit("a"), Err("Expected: digits".to_string()));
        assert_eq!(extract_digit("\r\n"), Err("Expected: digits".to_string()));
        assert_eq!(extract_digit("."), Err("Expected: digits".to_string()));
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digit("1+2"), Ok(("+2", "1")));
        assert_eq!(extract_digit("1 + 2"), Ok((" + 2", "1")));
    }

    #[test]
    fn extract_long_string() {
        assert_eq!(
            extract_string(
                "hello, world! This is a very long message. I guess I could've used lorem ipsum..."
            ),
            (
                "",
                "hello, world! This is a very long message. I guess I could've used lorem ipsum..."
            )
        );
    }
}
