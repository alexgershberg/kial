use crate::env::Env;
use crate::expr::binding_usage::BindingUsage;
use crate::expr::block::Block;
use crate::expr::operation::Operation;
use crate::utils;
use crate::val::Val;

pub mod binding_usage;
pub mod block;
pub mod operation;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digit(s)?;
        let number = number.parse().unwrap();

        Ok((s, Self(number)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Operation(Operation),
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, num)| (s, Self::Number(num)))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        Operation::new(s).map(|(s, op)| (s, Self::Operation(op)))
    }

    fn new_binding_usage(s: &str) -> Result<(&str, Self), String> {
        let (s, binding_usage) = BindingUsage::new(s)?;

        Ok((s, Self::BindingUsage(binding_usage)))
    }

    fn new_block(s: &str) -> Result<(&str, Self), String> {
        let (s, block) = Block::new(s)?;

        Ok((s, Self::Block(block)))
    }

    pub(crate) fn new_without_operation(s: &str) -> Result<(&str, Self), String> {
        let s = s.trim();

        if s.is_empty() {
            return Err("Empty expression".to_string());
        }

        let res1 = Self::new_binding_usage(s);

        let res2 = res1.or_else(|_| Self::new_block(s));

        let res3 = res2.or_else(|_| Self::new_number(s));

        res3
    }

    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = s.trim();

        if s.is_empty() {
            return Err("Empty expression".to_string());
        }

        let res1 = Self::new_operation(s);

        let res2 = res1.or_else(|_| Self::new_binding_usage(s));

        let res3 = res2.or_else(|_| Self::new_block(s));

        let res4 = res3.or_else(|_| Self::new_number(s));

        res4
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
            Self::Number(Number(num)) => Ok(Val::Number(*num)),

            Self::Operation(Operation { lhs, rhs, op }) => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                if let (Val::Number(lhs), Val::Number(rhs)) = (&lhs, &rhs) {
                    let result = match op {
                        Op::Add => lhs + rhs,
                        Op::Sub => lhs - rhs,
                        Op::Mul => lhs * rhs,
                        Op::Div => lhs / rhs,
                    };

                    return Ok(Val::Number(result));
                }

                Err(format!(
                    "Unsupported operation, lhs: {lhs:?} | rhs: {rhs:?} | op: {op:?}"
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::binding_usage::BindingUsage;

    #[test]
    fn expr_parse_gibberish_1() {
        assert_eq!(Expr::new("-"), Err("Expected: digits".to_string()))
    }

    #[test]
    fn parse_single_bracket() {
        assert_eq!(Expr::new("}"), Err("Expected: digits".to_string()))
    }

    #[test]
    fn parse_empty_expr() {
        assert_eq!(Expr::new(""), Err("Empty expression".to_string()))
    }

    #[test]
    fn parse_whitespace_expr() {
        assert_eq!(Expr::new("\r"), Err("Empty expression".to_string()));
        assert_eq!(Expr::new("\t"), Err("Empty expression".to_string()));
        assert_eq!(Expr::new(" "), Err("Empty expression".to_string()));

        assert_eq!(
            Expr::new("\r\n     \r\n"),
            Err("Empty expression".to_string())
        );

        assert_eq!(Expr::new("\r\n"), Err("Empty expression".to_string()))
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                })
            ))
        )
    }

    #[test]
    fn parse_op_on_variables_usage() {
        assert_eq!(
            Expr::new("a * b"),
            Ok((
                "",
                Expr::Operation(Operation {
                    lhs: Box::new(Expr::BindingUsage(BindingUsage {
                        name: "a".to_string()
                    })),

                    rhs: Box::new(Expr::BindingUsage(BindingUsage {
                        name: "b".to_string()
                    })),

                    op: Op::Mul
                })
            ))
        )
    }

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("128"), Ok(("", Number(128))));
    }

    #[test]
    fn parse_number_whitespace() {
        assert_eq!(Number::new(""), Err("Expected: digits".to_string()));
    }

    #[test]
    fn parse_number_gibberish() {
        assert_eq!(Number::new("-"), Err("Expected: digits".to_string()));
    }

    #[test]
    fn parse_ops() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_complex_ops() {
        assert_eq!(Op::new("+ 20"), Ok((" 20", Op::Add)));
    }

    #[test]
    fn parse_empty_ops() {
        assert_eq!(Op::new(""), Err(String::from("Expected: /")));
        assert_eq!(Op::new("20"), Err(String::from("Expected: /")));
    }

    #[test]
    fn parse_expression_as_operation() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation(Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Add
                })
            ))
        );

        assert_eq!(
            Expr::new("    \r\n    120     +     350"),
            Ok((
                "",
                Expr::Operation(Operation {
                    lhs: Box::new(Expr::Number(Number(120))),
                    rhs: Box::new(Expr::Number(Number(350))),
                    op: Op::Add
                })
            ))
        );

        assert_eq!(
            Expr::new("5 + -10"),
            Ok((
                "",
                Expr::Operation(Operation {
                    lhs: Box::new(Expr::Number(Number(5))),
                    rhs: Box::new(Expr::Number(Number(-10))),
                    op: Op::Add
                })
            ))
        )
    }

    #[test]
    fn parse_expression_as_number() {
        assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))));
    }

    #[test]
    fn eval_add() {
        let env = Env::default();

        assert_eq!(
            Expr::Operation(Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(30))),
                op: Op::Add
            })
            .eval(&env),
            Ok(Val::Number(40))
        );

        assert_eq!(
            Expr::Operation(Operation {
                lhs: Box::new(Expr::Number(Number(-250))),
                rhs: Box::new(Expr::Number(Number(100))),
                op: Op::Add
            })
            .eval(&env),
            Ok(Val::Number(-150))
        )
    }

    #[test]
    fn eval_sub() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation(Operation {
                lhs: Box::new(Expr::Number(Number(12))),
                rhs: Box::new(Expr::Number(Number(4))),
                op: Op::Sub
            })
            .eval(&env),
            Ok(Val::Number(8))
        )
    }

    #[test]
    fn eval_mul() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation(Operation {
                lhs: Box::new(Expr::Number(Number(3))),
                rhs: Box::new(Expr::Number(Number(4))),
                op: Op::Mul
            })
            .eval(&env),
            Ok(Val::Number(12))
        )
    }

    #[test]
    fn eval_div() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation(Operation {
                lhs: Box::new(Expr::Number(Number(12))),
                rhs: Box::new(Expr::Number(Number(4))),
                op: Op::Div
            })
            .eval(&env),
            Ok(Val::Number(3))
        )
    }
}
