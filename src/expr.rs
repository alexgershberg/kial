use crate::utils;
use crate::val::Val;

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
pub enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
}

impl Expr {
    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, num)| (s, Self::Number(num)))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespace(s);
        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);
        let (s, rhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_number(s))
    }

    pub(crate) fn eval(self) -> Val {
        match self {
            Self::Number(Number(num)) => Val::Number(num),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Val::Number(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("128"), Ok(("", Number(128))));
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
                Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                }
            ))
        );

        assert_eq!(
            Expr::new("    \r\n    120     +     350"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(120),
                    rhs: Number(350),
                    op: Op::Add
                }
            ))
        );

        assert_eq!(
            Expr::new("5 + -10"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Number(5),
                    rhs: Number(-10),
                    op: Op::Add
                }
            ))
        )
    }

    #[test]
    fn parse_expression_as_number() {
        assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))));
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(30),
                op: Op::Add
            }
            .eval(),
            Val::Number(40)
        );

        assert_eq!(
            Expr::Operation {
                lhs: Number(-250),
                rhs: Number(100),
                op: Op::Add
            }
            .eval(),
            Val::Number(-150)
        )
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(12),
                rhs: Number(4),
                op: Op::Sub
            }
            .eval(),
            Val::Number(8)
        )
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(3),
                rhs: Number(4),
                op: Op::Mul
            }
            .eval(),
            Val::Number(12)
        )
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(12),
                rhs: Number(4),
                op: Op::Div
            }
            .eval(),
            Val::Number(3)
        )
    }
}
