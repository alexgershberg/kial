use crate::ast::statement::Stmt;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    stmts: Vec<Stmt>,
}
