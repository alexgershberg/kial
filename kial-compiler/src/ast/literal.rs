use crate::lexer::TokenKind;
use crate::pear::Pear;

#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    String(String),
    Number(i32),
}

impl TryFrom<&mut Pear<'_>> for Literal {
    type Error = String;

    fn try_from(pear: &mut Pear) -> Result<Self, Self::Error> {
        let Some(token) = pear.peek_next() else {
            return Err("Expected literal, got \"None\"".to_string());
        };

        match token.kind {
            TokenKind::StringLiteral => {
                let token = pear.tag(TokenKind::StringLiteral)?;
                Ok(Literal::String(token.val))
            }
            TokenKind::NumericLiteral => {
                let token = pear.tag(TokenKind::NumericLiteral)?;
                let val = token.val.parse::<i32>().unwrap(); // This may panic, may need a more graceful way to handle types larger than i32
                Ok(Literal::Number(val))
            }

            other => Err(format!("Expected literal, got \"{:?}\"", other)),
        }
    }
}
