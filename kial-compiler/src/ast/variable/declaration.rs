use crate::ast::identifier::Ident;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct Declaration {
    name: Ident,
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

#[cfg(test)]
mod tests {
    use crate::ast::identifier::Ident;
    use crate::ast::variable::declaration::Declaration;
    use crate::pear::Pear;

    #[test]
    fn parse_declaration() {
        let mut pear = Pear::from("let a;");
        let local = Declaration::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Declaration {
                name: Ident("a".to_string())
            }
        );
    }
}
