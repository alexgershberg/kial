use crate::tokenstream::TokenStream;

#[derive(Debug, PartialEq)]
pub(crate) struct Ident(pub String);

impl Ident {
    fn parse(ts: &mut TokenStream) -> Result<Self, String> {
        Err("Expected Identifier, got... something else. TODO: better msg?".to_string())
    }
}
