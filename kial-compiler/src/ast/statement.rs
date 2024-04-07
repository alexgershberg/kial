use crate::ast::expression::Expr;
use crate::ast::local::Local;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    Local(Box<Local>),
    Expr(Box<Expr>),
    Semi(Box<Expr>), // TODO: Not sure what this is supposed to be
    Empty,
}
