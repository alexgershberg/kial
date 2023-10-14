use crate::env::Env;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;

        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;

        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(self, env: &mut Env) {
        env.store_binding(self.name, self.val.eval());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_bad_no_ident() {
        assert_eq!(
            BindingDef::new("let = 1+2"),
            Err(String::from("Expected: identifier"))
        )
    }

    #[test]
    fn parse_bad_no_space_after_let() {
        assert_eq!(
            BindingDef::new("letabcd=1+2"),
            Err(String::from("Expected: whitespace"))
        )
    }

    #[test]
    fn parse_binding_def_expression() {
        assert_eq!(
            BindingDef::new("let a = 10 / 5"),
            Ok((
                "",
                BindingDef {
                    name: String::from("a"),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(5),
                        op: Op::Div,
                    },
                }
            ))
        )
    }
}
