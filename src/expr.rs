use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digit(s);
        let number = number.parse().unwrap();

        (s, Number(number))
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
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            o => panic!("Invalid operator: {}", o),
        };

        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, _) = utils::extract_whitespace(s);
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);
        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespace(s);
        let (s, rhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);

        (s, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("128"), ("", Number(128)));
    }

    #[test]
    fn parse_ops() {
        assert_eq!(Op::new("+"), ("", Op::Add));
        assert_eq!(Op::new("-"), ("", Op::Sub));
        assert_eq!(Op::new("*"), ("", Op::Mul));
        assert_eq!(Op::new("/"), ("", Op::Div));
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                }
            )
        );

        assert_eq!(
            Expr::new("    \r\n    120     +     350"),
            (
                "",
                Expr {
                    lhs: Number(120),
                    rhs: Number(350),
                    op: Op::Add
                }
            )
        );

        assert_eq!(
            Expr::new("5 + -10"),
            (
                "",
                Expr {
                    lhs: Number(5),
                    rhs: Number(-10),
                    op: Op::Add
                }
            )
        )
    }
}
