use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expr::Expr;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Stmt {
    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let res = BindingDef::parse(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::parse(s).map(|(s, expr)| (s, Self::Expr(expr))));

        // Enforce ";" at EOL
        if let Ok((s, stmt)) = res {
            return match &stmt {
                Stmt::Expr(_) => Ok((s, stmt)),

                Stmt::BindingDef(_) => {
                    let s = utils::tag(";", s)?;
                    Ok((s, stmt))
                }
            };
        }

        res
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
    use crate::expr::block::Block;
    use crate::expr::operation::Operation;
    use crate::expr::{Expr, Number, Op};
    use crate::stmt::Stmt;

    // {let a = 20 let b = 10 let c = b + a c}
    #[test]
    fn parse_block() {
        assert_eq!(
            Stmt::parse("{let a = 20; let b = 10; let c = b + a; c}"),
            Ok((
                "",
                Stmt::Expr(Expr::Block(Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(20))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::Number(Number(10))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "c".to_string(),
                            val: Expr::Operation(Operation {
                                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                                    name: "b".to_string()
                                })),

                                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                                    name: "a".to_string()
                                })),
                                op: Op::Add,
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "c".to_string()
                        }))
                    ]
                }))
            ))
        );
    }

    #[test]
    fn parse_empty_statemnt() {
        assert_eq!(Stmt::parse("\n"), Err("Empty expression".to_string()));
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Stmt::parse("let a = 5;"),
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
            Stmt::parse("10 * 15"),
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
            Stmt::parse("10"),
            Ok(("", Stmt::Expr(Expr::Number(Number(10)))))
        );
    }

    #[test]
    fn parse_assigment_of_expr_to_variable() {
        assert_eq!(
            Stmt::parse("let a = 10 * 15;"),
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
            Stmt::parse("let a = b;"),
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
            Stmt::parse("let a = b / c;"),
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
