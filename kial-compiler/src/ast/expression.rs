use crate::ast::binary_operation::BinOp;
use crate::ast::block::Block;
use crate::ast::function::Fn;
use crate::ast::identifier::Ident;
use crate::ast::literal::LitKind;
use crate::ast::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Block(Box<Block>),
    Lit(LitKind),
    FnCall(Fn, Vec<Ident>),
}

impl TryFrom<&mut Pear<'_>> for Expr {
    type Error = String;

    fn try_from(value: &mut Pear) -> Result<Self, Self::Error> {
        todo!()
    }
}
