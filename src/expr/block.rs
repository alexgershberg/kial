use crate::env::Env;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub exprs: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("{", s)?;

        let (s, _) = utils::extract_whitespace(s);

        let (s, exprs) = if let Ok((s, stmt)) = Stmt::new(s) {
            (s, vec![stmt])
        } else {
            (s, Vec::new())
        };
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("}", s)?;
        let (s, _) = utils::extract_whitespace(s);

        Ok((s, Self { exprs }))
    }

    pub fn eval(self, env: &Env) -> Result<Val, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::{Expr, Number, Op};

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { exprs: Vec::new() })))
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(
            Block::new("{       }"),
            Ok(("", Block { exprs: Vec::new() }))
        );

        assert_eq!(Block::new(" {   } "), Ok(("", Block { exprs: Vec::new() })))
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    exprs: vec![Stmt::Expr(Expr::Number(Number(5)))]
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
                    exprs: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "".to_string(),
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
