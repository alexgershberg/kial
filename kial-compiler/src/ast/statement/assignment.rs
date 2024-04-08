use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct Assignment {
    pub(crate) name: Ident,
    pub(crate) value: Expr,
}

impl TryFrom<&mut Pear<'_>> for Assignment {
    type Error = String;
    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        let name = Ident::try_from(&mut *pear)?;
        pear.tag(TokenKind::Equals)?;
        let value = Expr::try_from(&mut *pear)?;
        pear.tag(TokenKind::Semi)?;
        Ok(Self { name, value })
    }
}

#[cfg(test)]
mod test {
    use crate::ast::expression::Expr;
    use crate::ast::identifier::Ident;
    use crate::ast::literal::Literal;
    use crate::ast::variable::assignment::Assignment;
    use crate::pear::Pear;

    #[test]
    fn parse_assignment() {
        let mut pear = Pear::from("c = 30");
        let local = Assignment::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Assignment {
                name: Ident("c".to_string()),
                value: Expr::Literal(Literal::Number(30))
            }
        );
    }
}
