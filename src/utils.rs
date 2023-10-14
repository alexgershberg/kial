pub(crate) fn extract_digit(s: &str) -> (&str, &str){
    take_while(|c|{c.is_ascii_digit()}, s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    take_while(|c|{ matches!(c, '+' | '-' | '*' | '/') }, s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c|{c.is_ascii_whitespace()}, s)
}

pub(crate) fn take_while(pred: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let char_end = s.char_indices()
        .find_map(|(idx, c)| {
            if pred(c) { None } else {Some(idx)}
        })
        .unwrap_or(s.len());

    let extracted = &s[..char_end];
    let remainder = &s[char_end..];

    (remainder, extracted)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_whitespace_basic() {
        assert_eq!(extract_whitespace("  \t   1"), ("1", "  \t   "));
        assert_eq!(extract_whitespace("     200"), ("200", "     "));
        assert_eq!(extract_whitespace(" 200\r\n"), ("200\r\n", " "));
        assert_eq!(extract_whitespace("\r\nHello World\r\n"), ("Hello World\r\n", "\r\n"));
    }

    #[test]
    fn extract_empty() {
        assert_eq!(extract_digit(""), ("", ""));
    }

    #[test]
    fn extract_big() {
        assert_eq!(extract_digit("123456+99999"), ("+99999", "123456"));
    }

    // Don't care for now
    // #[test]
    // fn extract_negative() {
    //     assert_eq!(extract_digit("-300+1"), ("+1", "-300"));
    // }

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