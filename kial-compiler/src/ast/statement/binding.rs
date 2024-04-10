use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct Declaration {
    pub(crate) name: Ident,
}

impl TryFrom<&mut Pear<'_>> for Declaration {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        pear.tag(TokenKind::Let)?;
        let name = Ident::try_from(&mut *pear)?; // Have to explicit re-borrow here https://quinedot.github.io/rust-learning/st-reborrow.html
        pear.tag(TokenKind::Semi)?;
        Ok(Self { name })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Initialization {
    pub(crate) name: Ident,
    pub(crate) value: Expr,
}

impl TryFrom<&mut Pear<'_>> for Initialization {
    type Error = String;
    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        pear.tag(TokenKind::Let)?;
        let name = Ident::try_from(&mut *pear)?;
        pear.tag(TokenKind::Equals)?;
        let value = Expr::try_from(&mut *pear)?;
        pear.tag(TokenKind::Semi)?;
        Ok(Self { name, value })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Binding {
    Declaration(Declaration),
    Initialization(Initialization),
}

impl TryFrom<&mut Pear<'_>> for Binding {
    type Error = String;

    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        pear.tag(TokenKind::Let)?;
        let name = Ident::try_from(&mut *pear)?;
        if pear
            .peek_next()
            .is_some_and(|token| token.kind == TokenKind::Semi)
        {
            return Ok(Self::Declaration(Declaration { name }));
        }

        pear.tag(TokenKind::Equals)?;
        let value = Expr::try_from(&mut *pear)?;
        Ok(Self::Initialization(Initialization { name, value }))
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: Ident,
}

impl TryFrom<&mut Pear<'_>> for BindingUsage {
    type Error = String;

    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        let name = Ident::try_from(&mut *pear)?;
        Ok(Self { name })
    }
}
