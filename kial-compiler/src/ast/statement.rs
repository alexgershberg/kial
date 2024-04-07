use crate::ast::expression::Expr;
use crate::ast::local::Local;

pub(crate) enum Stmt {
    Local(Box<Local>),
    Expr(Box<Expr>),
    Semi(Box<Expr>),
    Empty,
}
