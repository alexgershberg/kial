use crate::ast::block::Block;
use crate::ast::identifier::Ident;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDefinition {
    name: Ident,
    body: Block,
}

#[derive(Debug, PartialEq)]
enum Arg {
    Identifier,
    Literal,
}

#[derive(Debug, PartialEq)]
struct ArgumentList {
    args: Vec<Arg>,
}

impl TryFrom<&mut Pear<'_>> for ArgumentList {
    type Error = String;
    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        unreachable!()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionInvocation {
    name: Ident,
    arguments: ArgumentList,
}

impl TryFrom<&mut Pear<'_>> for FunctionInvocation {
    type Error = String;

    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        let name = Ident::try_from(&mut *pear)?;
        let arguments = ArgumentList::try_from(&mut *pear)?;

        Ok(Self { name, arguments })
    }
}
