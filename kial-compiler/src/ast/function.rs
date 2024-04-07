use crate::ast::block::Block;
use crate::ast::identifier::Ident;

#[derive(Debug, PartialEq)]
pub(crate) struct Fn {
    ident: Ident,
    block: Block,
}
