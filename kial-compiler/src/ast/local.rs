use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;
use crate::ast::local::Local::Init;
use crate::ast::pear::Pear;
use crate::lexer::TokenKind;

#[derive(Debug, PartialEq)]
pub(crate) enum Local {
    Decl { ident: Ident },
    Init { ident: Ident, val: Expr },
}

impl TryFrom<&mut Pear<'_>> for Local {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        pear.extract_whitespace()?;
        pear.tag(TokenKind::Let)?;
        pear.extract_whitespace()?;
        let ident = Ident::try_from(&mut *pear)?; // Have to explicit re-borrow here https://quinedot.github.io/rust-learning/st-reborrow.html

        if pear.tag(TokenKind::Semi).is_ok() {
            return Ok(Local::Decl { ident });
        }

        let val = Expr::try_from(&mut *pear)?;

        Ok(Init { ident, val })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::identifier::Ident;
    use crate::ast::local::Local;
    use crate::ast::pear::Pear;

    #[test]
    fn basic_usage() {
        let mut pear = Pear::from("let a;");
        let local = Local::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Local::Decl {
                ident: Ident("a".to_string())
            }
        );
    }
}
