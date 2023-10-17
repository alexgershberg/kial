use crate::expr::{Expr, Op};
use crate::utils;

#[derive(Debug, PartialEq)]
pub(crate) struct Operation {
    pub(crate) lhs: Box<Expr>,
    pub(crate) rhs: Box<Expr>,
    pub(crate) op: Op,
}

impl Operation {
    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);

        let (s, lhs) = Expr::parse_without_operation(s)?;

        let (s, _) = utils::extract_whitespace(s);
        let (s, op) = Op::parse(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Expr::parse_without_operation(s)?;

        let (s, _) = utils::extract_whitespace(s);

        Ok((
            s,
            Self {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::operation::Operation;

    #[test]
    fn op_parse_gibberish_1() {
        assert_eq!(Operation::parse("-"), Err("Expected: digits".to_string()))
    }
}
