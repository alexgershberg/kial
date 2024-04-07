use crate::ast::block::Block;
use crate::ast::identifier::Ident;

pub(crate) struct Fn {
    ident: Ident,
    block: Block,
}
