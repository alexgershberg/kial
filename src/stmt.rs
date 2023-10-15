use crate::binding_def::BindingDef;
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }
}

#[cfg(test)]
mod tests {
    use crate::binding_def::BindingDef;
    use crate::expr::{Expr, Number, Op};
    use crate::stmt::Stmt;

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::new("let a = 5"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(5)),
                })
            ))
        );
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Stmt::new("10 * 15"),
            Ok((
                "",
                Stmt::Expr(Expr::Operation {
                    lhs: Number(10),
                    rhs: Number(15),
                    op: Op::Mul,
                })
            ))
        );
    }

    #[test]
    fn parse_single_number() {
        assert_eq!(
            Stmt::new("10"),
            Ok(("", Stmt::Expr(Expr::Number(Number(10)))))
        );
    }
}
