use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;

pub(crate) enum Local {
    Decl { ident: Ident },
    Init { ident: Ident, val: Box<Expr> },
}
