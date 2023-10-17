use crate::expr::{Expr, Number, Op};
use crate::utils;
use std::fmt::format;

#[derive(Debug, PartialEq)]
pub(crate) struct Operation {
    pub(crate) lhs: Box<Expr>,
    pub(crate) rhs: Box<Expr>,
    pub(crate) op: Op,
}

impl Operation {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        if s.starts_with("}") {
            // TODO: Try re-running the tests without this check?
            return Err(format!("Invalid symbol"));
        }

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
