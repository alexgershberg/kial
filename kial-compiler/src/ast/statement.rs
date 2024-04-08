use crate::ast::expression::Expr;
use crate::ast::statement::assignment::Assignment;
use crate::lexer::TokenKind;
use crate::pear::Pear;
use binding::Binding;

mod assignment;
pub(crate) mod binding;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    Assignment(Assignment),
    Binding(Binding),
    Expr(Expr),
}

impl TryFrom<&mut Pear<'_>> for Stmt {
    type Error = String;

    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        let Some(next) = pear.peek_next() else {
            return Err("Expected token, found None".to_string());
        };

        if let Ok(var) = Binding::try_from(&mut *pear) {
            pear.tag(TokenKind::Semi)?;
            return Ok(Stmt::Binding(var));
        }

        let next_next = pear.peek_n(2);
        let is_assignment = next.kind == TokenKind::Ident
            && next_next.is_some_and(|token| token.kind == TokenKind::Equals);
        if is_assignment {
            if let Ok(assign) = Assignment::try_from(&mut *pear) {
                pear.tag(TokenKind::Semi)?;
                return Ok(Stmt::Assignment(assign));
            }
        }

        if let Ok(expr) = Expr::try_from(&mut *pear) {
            return Ok(Self::Expr(expr));
        }

        Err("Malformed statement".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expr;
    use crate::ast::identifier::Ident;
    use crate::ast::literal::Literal;
    use crate::ast::statement::assignment::Assignment;
    use crate::ast::statement::binding::{Binding, BindingUsage, Declaration, Initialization};
    use crate::ast::statement::Stmt;
    use crate::pear::Pear;

    #[test]
    fn statement_declaration() {
        let mut pear = Pear::from("let a;");
        let stmt = Stmt::try_from(&mut pear).unwrap();

        assert_eq!(
            stmt,
            Stmt::Binding(Binding::Declaration(Declaration {
                name: Ident("a".to_string()),
            }))
        );
    }

    #[test]
    fn statement_initialization() {
        let mut pear = Pear::from("let b = 25;");
        let stmt = Stmt::try_from(&mut pear).unwrap();

        assert_eq!(
            stmt,
            Stmt::Binding(Binding::Initialization(Initialization {
                name: Ident("b".to_string()),
                value: Expr::Literal(Literal::Number(25)),
            }))
        );
    }

    #[test]
    fn statement_assignment() {
        let mut pear = Pear::from("c = \"Anything else\";");
        let stmt = Stmt::try_from(&mut pear).unwrap();

        assert_eq!(
            stmt,
            Stmt::Assignment(Assignment {
                name: Ident("c".to_string()),
                value: Expr::Literal(Literal::String("\"Anything else\"".to_string())),
            })
        );
    }

    #[test]
    fn statement_binding_usage() {
        let mut pear = Pear::from("c");
        let stmt = Stmt::try_from(&mut pear);
        assert_eq!(
            stmt,
            Ok(Stmt::Expr(Expr::BindingUsage(BindingUsage {
                name: Ident("c".to_string())
            })))
        )
    }
}
