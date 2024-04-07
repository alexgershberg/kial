use crate::tokenstream::TokenStream;

#[derive(Debug, PartialEq)]
pub(crate) enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn parse(ts: TokenStream) -> Result<Self, String> {
        todo!()
    }
}
