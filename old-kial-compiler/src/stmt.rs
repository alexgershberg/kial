use crate::env::Env;
use crate::expr::Expr;
use crate::stmt::assignment::Assignment;
use crate::stmt::binding_def::BindingDef;
use crate::stmt::function_def::FunctionDef;
use crate::val::Val;

pub mod assignment;
pub mod binding_def;
mod function_def;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    BindingDef(BindingDef),
    Expr(Expr),
    Assignment(Assignment),
    FunctionDef(FunctionDef),
}

impl Stmt {
    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let s = s.trim();

        if s.is_empty() {
            return Err("Empty".to_string());
        }

        BindingDef::parse(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| {
                Assignment::parse(s).map(|(s, assignment)| (s, Self::Assignment(assignment)))
            })
            .or_else(|_| Expr::parse(s).map(|(s, expr)| (s, Self::Expr(expr))))
            .or_else(|_| {
                FunctionDef::parse(s).map(|(s, func_def)| (s, Self::FunctionDef(func_def)))
            })
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => binding_def.eval(env),
            Self::Expr(expr) => expr.eval(env),
            Self::Assignment(assignment) => assignment.eval(env),
            Self::FunctionDef(func_def) => func_def.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::block::Block;
    use crate::expr::operation::Operation;
    use crate::expr::{Expr, Number, Op, Str};
    use crate::stmt::binding_def::BindingDef;
    use crate::stmt::function_def::FunctionDef;
    use crate::stmt::Stmt;

    #[test]
    fn parse_function() {
        assert_eq!(
            Stmt::parse("func main() {}"),
            Ok((
                "",
                Stmt::FunctionDef(FunctionDef::new("main", vec![], Block::new(vec![])))
            ))
        );
    }

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
    fn parse_empty_statement() {
        assert_eq!(Stmt::parse("\n"), Err("Empty".to_string()));
    }

    #[ignore] // Don't think current architecture supports this
    #[test]
    fn parse_missing_semicolon() {
        assert_eq!(
            Stmt::parse("let count = 10"),
            Err("Expected: ;".to_string())
        );
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
    fn parse_assigment_of_string_to_variable() {
        assert_eq!(
            Stmt::parse(r#"let a = "hello world";"#),
            Ok((
                "",
                Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Str(Str("hello world".to_string()))
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
