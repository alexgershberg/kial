use crate::ast::binary_operation::BinOp;
use crate::ast::block::Block;
use crate::ast::function::FunctionInvocation;
use crate::ast::literal::Literal;
use crate::ast::statement::binding::BindingUsage;
use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Binary(Box<Expr>, Box<Expr>, BinOp),
    Block(Block),
    Literal(Literal),
    BindingUsage(BindingUsage),
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
        if is_function {
            let func = FunctionInvocation::try_from(&mut *pear)?;
            return Ok(Self::Function(func));
        }

        let is_ident = next.kind == TokenKind::Ident;
        if is_ident {
            let binding = BindingUsage::try_from(&mut *pear)?;
            return Ok(Self::BindingUsage(binding));
        }

        let is_block = next.kind == TokenKind::OpenBrace;
        if is_block {
            let block = Block::try_from(&mut *pear)?;
            return Ok(Self::Block(block));
        }
        // TODO: Finish other variants

        let literal = Literal::try_from(&mut *pear)?;
        return Ok(Self::Literal(literal));
    }
}
