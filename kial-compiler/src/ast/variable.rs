use crate::ast::expression::Expr;
use crate::ast::identifier::Ident;
use crate::ast::local::Variable::Initialization;
use crate::ast::pear::Pear;
use crate::lexer::TokenKind;

#[derive(Debug, PartialEq)]
pub(crate) enum Variable {
    Declaration { ident: Ident },
    Initialization { ident: Ident, val: Expr },
}

impl TryFrom<&mut Pear<'_>> for Variable {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        pear.extract_whitespace()?;
        pear.tag(TokenKind::Let)?;
        pear.extract_whitespace()?;
        let ident = Ident::try_from(&mut *pear)?; // Have to explicit re-borrow here https://quinedot.github.io/rust-learning/st-reborrow.html

        if pear.tag(TokenKind::Semi).is_ok() {
            return Ok(Variable::Declaration { ident });
        }

        pear.tag(TokenKind::Equals)?;

        let val = Expr::try_from(&mut *pear)?;

        Ok(Initialization { ident, val })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expr;
    use crate::ast::identifier::Ident;
    use crate::ast::literal::Literal;
    use crate::ast::local::Variable;
    use crate::ast::pear::Pear;

    #[test]
    fn parse_declaration() {
        let mut pear = Pear::from("let a;");
        let local = Variable::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Variable::Declaration {
                ident: Ident("a".to_string())
            }
        );
    }

    #[test]
    fn parse_initialization() {
        let mut pear = Pear::from("let b = 10;");
        let local = Variable::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Variable::Initialization {
                ident: Ident("b".to_string()),
                val: Expr::Literal(Literal::Number(10))
            }
        );
    }

    #[test]
    fn parse_assignment() {
        let mut pear = Pear::from("c = 30;");
        let local = Variable::try_from(&mut pear).unwrap();
        assert_eq!(
            local,
            Variable::Initialization {
                ident: Ident("c".to_string()),
                val: Expr::Literal(Literal::Number(30))
            }
        );
    }
}
