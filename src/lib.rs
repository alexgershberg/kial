mod utils;

#[derive(Debug, PartialEq)]
struct Number(i32);

impl Number {
    fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digit(s);
        let number = number.parse().unwrap();

        (s, Number(number))
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn new(s: &str) -> (&str, Self) {
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
struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr {
    fn new(s: &str) -> (&str, Self) {
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
    fn parse_add_op() {
        assert_eq!(Op::new("+"), ("", Op::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), ("", Op::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), ("", Op::Div));
    }

    #[test]
    fn parse_one_plus_two() {
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
        )
    }
}
