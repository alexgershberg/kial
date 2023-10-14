use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    fn new(s: &str) -> (&str, Self) {
        let s = s.strip_prefix("let").expect("Expected: let");

        let (s, _) = utils::extract_whitespace(s);
        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespace(s);

        let s = s.strip_prefix('=').expect("Expected: =");

        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 5"),
            (
                "",
                BindingDef {
                    name: String::from("a"),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(5),
                        op: Op::Div,
                    },
                }
            )
        )
    }
}
