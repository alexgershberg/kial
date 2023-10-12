
pub(crate) fn extract_digit(s: &str) -> (&str, &str){
    let digits_end = s.char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() {None} else {Some(idx)})
        .unwrap_or(s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];

    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("Invalid operator")
    };

    (&s[1..], &s[0..1])
}


#[cfg(test)]
mod test {
    use super::*;

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
    fn extract_add_op() {
        assert_eq!(extract_op("+20"), ("20", "+"));
    }

    #[test]
    fn extract_sub_op() {
        assert_eq!(extract_op("-100"), ("100", "-"));
    }

    #[test]
    fn extract_mul_op() {
        assert_eq!(extract_op("*4"), ("4", "*"));
    }

    #[test]
    fn extract_div_op() {
        assert_eq!(extract_op("/33"), ("33", "/"));
    }
}