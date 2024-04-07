use crate::ast::expression::Expr;
use crate::ast::variable::assignment::Assignment;
use crate::ast::variable::declaration::Declaration;
use crate::ast::variable::initialization::Initialization;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    Declaration(Declaration),
    Assignment(Assignment),
    Initialization(Initialization), // TODO: Could these 3 variable enums be replaced with Variable(Decl, Init, Assign) enum instead?
    Expr(Box<Expr>),
    Empty,
}
