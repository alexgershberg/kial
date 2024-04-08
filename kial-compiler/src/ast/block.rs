use crate::ast::statement::Stmt;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    stmts: Vec<Stmt>,
}

impl TryFrom<&mut Pear<'_>> for Block {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        let mut stmts = vec![];

        pear.tag(TokenKind::OpenBrace)?;
        while let Ok(stmt) = Stmt::try_from(&mut *pear) {
            stmts.push(stmt);
        }

        pear.tag(TokenKind::CloseBrace)?;

        Ok(Self { stmts })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::block::Block;
    use crate::pear::Pear;

    #[test]
    fn basic_block_test() {
        let mut pear = Pear::from(
            "{\
let a = 10;
let b = 20;

a
        }",
        );
        let block = Block::try_from(&mut pear);
        println!("{block:#?}")
    }
}
