use crate::ast::function::{FunctionDefinition, FunctionInvocation};
use crate::tokenstream::TokenStream;

// TODO: WIP
pub(crate) struct Module {
    entry: Option<FunctionDefinition>,
    functions: Vec<FunctionInvocation>,
}

impl Module {
    fn parse(ts: TokenStream) -> Self {
        todo!()
    }
}
