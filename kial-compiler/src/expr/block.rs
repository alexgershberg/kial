use crate::env::Env;

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
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }

        let mut env = env.create_child();

        // We don't need to eval last stmt here (we evaluate it at end), but this still works, so...
        for stmt in &self.stmts {
            stmt.eval(&mut env)?;
        }

        self.stmts.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::operation::Operation;
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
    fn eval_bad_block() {
        /*
        {
            a
        }
        */
        let env = Env::default();
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::BindingUsage(BindingUsage {
                    name: "a".to_string()
                }))]
            }
            .eval(&env),
            Err("Binding does not exist: a".to_string())
        )
    }

    #[test]
    fn eval_block_with_single_assignment() {
        /*
        {
            let a = 10
        }
        */
        let env = Env::default();
        assert_eq!(
            Block {
                stmts: vec![Stmt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                })]
            }
            .eval(&env),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_block_with_multiple_assignments() {
        /*
        {
            let a = 10
            let b = 20
            let c = 30
        }
        */
        let env = Env::default();
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "a".to_string(),
                        val: Expr::Number(Number(10)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "b".to_string(),
                        val: Expr::Number(Number(20)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "c".to_string(),
                        val: Expr::Number(Number(30)),
                    })
                ]
            }
            .eval(&env),
            Ok(Val::Unit)
        )
    }

    // #[test]
    // fn parse_complicated_block() {
    //     /*
    //     {
    //         let a = 3000 + 500
    //         let b = 350
    //         a = a + b
    //         a
    //     }
    //     */
    //
    //     assert_eq!(
    //         Block::new(
    //             "
    //     {
    //         let a = 3000 + 500
    //         let b = 350
    //         a = a + b  // This isn't supported yet!!
    //         a
    //     }
    //     "
    //         ),
    //         Ok((
    //             "",
    //             Block {
    //                 stmts: vec![
    //                     Stmt::Expr(Expr::Operation(Operation {
    //                         lhs: Box::new(Expr::Number(Number(3000))),
    //                         rhs: Box::new(Expr::Number(Number(500))),
    //                         op: Op::Add,
    //                     })),
    //                     Stmt::BindingDef(BindingDef {
    //                         name: "b".to_string(),
    //                         val: Expr::Number(Number(350))
    //                     }),
    //                     Stmt::BindingDef(BindingDef {
    //                         name: "b".to_string(),
    //                         val: Expr::Number(Number(350))
    //                     }),
    //                 ]
    //             }
    //         ))
    //     )
    // }

    #[test]
    fn eval_block_with_outer_variables() {
        /*
        let baz = 2
        {
            let foo = baz
            foo
        }
        */

        let mut env = Env::default();
        env.store_binding("baz".to_string(), Val::Number(2));

        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::BindingUsage(BindingUsage {
                            name: "baz".to_string()
                        })
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "foo".to_string()
                    }))
                ]
            }
            .eval(&env),
            Ok(Val::Number(2))
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
                            val: Expr::Operation(Operation {
                                lhs: Box::new(Expr::Number(Number(20))),
                                rhs: Box::new(Expr::Number(Number(4))),
                                op: Op::Div,
                            })
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
