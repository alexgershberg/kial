use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;
use crate::lexer::TokenKind;
use crate::pear::Pear;

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

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expr;
    use crate::ast::identifier::Ident;
    use crate::ast::literal::Literal;
    use crate::ast::variable::initialization::Initialization;
    use crate::pear::Pear;

    #[test]
    fn parse_initialization() {
        let mut pear = Pear::from("let b = 10");
        let local = Initialization::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Initialization {
                name: Ident("b".to_string()),
                value: Expr::Literal(Literal::Number(10))
            }
        );
    }
}
