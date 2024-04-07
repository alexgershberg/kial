use crate::ast::function::Fn;
use crate::tokenstream::TokenStream;

pub(crate) struct Module {
    entry: Option<Fn>,
    functions: Vec<Fn>,
}

impl Module {
    fn parse(ts: TokenStream) -> Self {
        todo!()
    }
}
