use crate::env::Env;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("{", s)?;

        let (s, _) = utils::extract_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();
        while let Ok((new_s, stmt)) = Stmt::new(s) {
            s = new_s;
            stmts.push(stmt)
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;
        let (s, _) = utils::extract_whitespace(s);

        Ok((s, Self { stmts }))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        self.stmts.last().map_or(Ok(Val::Unit), |stmt| match stmt {
            Stmt::Expr(expr) => expr.eval(&env),
            Stmt::BindingDef(binding_def) => todo!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::{Expr, Number, Op};

    #[test]
    fn eval_empty_block() {
        let env = Env::default();
        assert_eq!(Block { stmts: Vec::new() }.eval(&env), Ok(Val::Unit))
    }

    #[test]
    fn eval_single_number_block() {
        /*
        {
            10
        }
        */
        let env = Env::default();
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(10)))]
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_simple_block() {
        /*
        {
            let a = 10
            a
        }
        */
        let env = Env::default();
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "a".to_string(),
                        val: Expr::Number(Number(10))
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "a".to_string()
                    }))
                ]
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(
            Block::new("{       }"),
            Ok(("", Block { stmts: Vec::new() }))
        );

        assert_eq!(Block::new(" {   } "), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]
                }
            ))
        )
    }

    #[test]
    fn parse_block_multiple_stmts() {
        assert_eq!(
            Block::new(
                "{
                let a = 20 / 4
                let b = a
                a
                }"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Operation {
                                lhs: Number(20),
                                rhs: Number(4),
                                op: Op::Div,
                            }
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string(),
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "a".to_string(),
                        }))
                    ]
                }
            ))
        )
    }
}
