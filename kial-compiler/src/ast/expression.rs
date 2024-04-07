use crate::ast::binary_operation::BinOp;
use crate::ast::block::Block;
use crate::ast::function::Fn;
use crate::ast::identifier::Ident;
use crate::ast::literal::LitKind;
use crate::tokenstream::TokenStream;

pub(crate) enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Block(Box<Block>),
    Lit(LitKind),
    FnCall(Fn, Vec<Ident>),
}

impl Expr {
    fn parse(ts: &mut TokenStream) -> Self {
        todo!()
    }
}
