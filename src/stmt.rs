use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expr::Expr;
use crate::val::Val;

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

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => binding_def.eval(env),
            Self::Expr(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::operation::Operation;
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
                Stmt::Expr(Expr::Operation(Operation {
                    lhs: Box::new(Expr::Number(Number(10))),
                    rhs: Box::new(Expr::Number(Number(15))),
                    op: Op::Mul
                }))
            ))
        )
    }

    #[test]
    fn parse_single_number() {
        assert_eq!(
            Stmt::new("10"),
            Ok(("", Stmt::Expr(Expr::Number(Number(10)))))
        );
    }

    #[test]
    fn parse_assigment_of_expr_to_variable() {
        assert_eq!(
            Stmt::new("let a = 10 * 15"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation(Operation {
                        lhs: Box::new(Expr::Number(Number(10))),
                        rhs: Box::new(Expr::Number(Number(15))),
                        op: Op::Mul
                    })
                })
            ))
        );
    }

    #[test]
    fn parse_assigment_of_variable_to_variable() {
        assert_eq!(
            Stmt::new("let a = b"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::BindingUsage(BindingUsage {
                        name: "b".to_string()
                    })
                })
            ))
        );
    }

    #[test]
    fn parse_assigment_of_multi_variable_expression_to_variable() {
        assert_eq!(
            Stmt::new("let a = b / c"),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation(Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "c".to_string()
                        })),
                        op: Op::Div
                    })
                })
            ))
        );
    }
}
