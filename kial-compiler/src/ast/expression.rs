use crate::ast::binary_operation::BinOp;
use crate::ast::block::Block;
use crate::ast::function::FunctionInvocation;
use crate::ast::literal::Literal;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Block(Box<Block>),
    Literal(Literal),
    Function(FunctionInvocation),
}

impl TryFrom<&mut Pear<'_>> for Expr {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        let Some(next) = pear.peek_next() else {
            return Err("TODO: No tokens found".to_string()); // TODO
        };

        // TODO: Reverse Polish Notation here?

        let is_literal =
            (next.kind == TokenKind::StringLiteral) | (next.kind == TokenKind::NumericLiteral);
        if is_literal {
            let literal = Literal::try_from(&mut *pear)?;
            return Ok(Self::Literal(literal));
        }

        let is_function = next.kind == TokenKind::Func;

        // TODO: Finish other variants

        let func = FunctionInvocation::try_from(&mut *pear)?;
        return Ok(Self::Function(func));
    }
}
