use crate::ast::expression::Expr;
use crate::ast::variable::assignment::Assignment;
use crate::ast::variable::declaration::Declaration;
use crate::ast::variable::initialization::Initialization;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) enum Stmt {
    Declaration(Declaration),
    Assignment(Assignment),
    Initialization(Initialization), // TODO: Could these 3 variable enums be replaced with Variable(Decl, Init, Assign) enum instead?
    Expr(Box<Expr>),
    Empty,
}

impl TryFrom<&mut Pear<'_>> for Stmt {
    type Error = String;

    fn try_from(pear: &mut Pear<'_>) -> Result<Self, Self::Error> {
        if let Ok(assign) = Assignment::try_from(&mut *pear) {
            return Ok(Stmt::Assignment(assign));
        }

        if let Ok(init) = Initialization::try_from(&mut *pear) {
            return Ok(Stmt::Initialization(init));
        }

        if let Ok(decl) = Declaration::try_from(&mut *pear) {
            return Ok(Stmt::Declaration(decl));
        }

        Ok(Stmt::Empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expr;
    use crate::ast::identifier::Ident;
    use crate::ast::literal::Literal;
    use crate::ast::statement::Stmt;
    use crate::ast::variable::assignment::Assignment;
    use crate::ast::variable::declaration::Declaration;
    use crate::ast::variable::initialization::Initialization;
    use crate::pear::Pear;

    #[test]
    fn statement_declaration() {
        let mut pear = Pear::from("let a;");
        let stmt = Stmt::try_from(&mut pear).unwrap();

        assert_eq!(
            stmt,
            Stmt::Declaration(Declaration {
                name: Ident("a".to_string()),
            })
        );
    }

    #[test]
    fn statement_initialization() {
        let mut pear = Pear::from("let b = 25;");
        let stmt = Stmt::try_from(&mut pear).unwrap();

        assert_eq!(
            stmt,
            Stmt::Initialization(Initialization {
                name: Ident("b".to_string()),
                value: Expr::Literal(Literal::Number(25)),
            })
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
}
