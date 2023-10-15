use crate::expr::{Expr, Number, Op};
use crate::utils;

#[derive(Debug, PartialEq)]
pub(crate) struct Operation {
    pub(crate) lhs: Box<Expr>,
    pub(crate) rhs: Box<Expr>,
    pub(crate) op: Op,
}

impl Operation {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, _) = utils::extract_whitespace(s);

        let (s, lhs) = Expr::new_number(s)
            .or_else(|_| Expr::new_binding_usage(s))
            .or_else(|_| Expr::new(s))?;

        let (s, _) = utils::extract_whitespace(s);
        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Expr::new_number(s)
            .or_else(|_| Expr::new_binding_usage(s))
            .or_else(|_| Expr::new(s))?;

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
