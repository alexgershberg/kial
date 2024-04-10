use crate::lexer::{Token, TokenKind};
use crate::tokenstream::TokenStream;

pub(crate) struct Pear<'a> {
    ts: TokenStream<'a>,
}

impl Pear<'_> {
    fn take_while(&mut self, pred: fn(Token) -> bool) {
        while let Some(next) = self.peek_next() {
            if !pred(next) {
                return;
            }

            self.consume_1();
        }
    }

    pub(crate) fn extract_whitespace(&mut self) -> Result<(), String> {
        self.take_while(|token| token.kind == TokenKind::Whitespace);
        Ok(())
    }

    pub(crate) fn extract_identifier(&mut self) -> Result<Token, String> {
        self.take_1(|token| token.kind == TokenKind::Ident)
    }

    pub(crate) fn tag(&mut self, token_kind: TokenKind) -> Result<Token, String> {
        let Some(actual) = self.peek_next() else {
            return Err(format!("Expected \"{token_kind:?}\" but got \"None\""));
        };

        if actual.kind == token_kind {
            Ok(self.consume_1().unwrap()) // Safe, as we checked peek_next() above
        } else {
            Err(format!(
                "Expected \"{token_kind:?}\" but got \"{:?}\"",
                actual.kind
            ))
        }
    }

    fn take_1(&mut self, pred: fn(&Token) -> bool) -> Result<Token, String> {
        let Some(token) = self.peek_next() else {
            return Err("Expected token, but found None".to_string());
        };

        if pred(&token) {
            Ok(self.consume_1().unwrap()) // Shouldn't panic as we checked next in peek_next());
        } else {
            Err(format!("Unexpected token: \"{}\"", token))
        }
    }

    fn consume_1(&mut self) -> Option<Token> {
        self.ts.next()
    }

    pub(crate) fn peek_next(&mut self) -> Option<Token> {
        self.ts.peek_next()
    }

    pub(crate) fn peek_n(&mut self, n: usize) -> Option<Token> {
        self.ts.peek_n(n)
    }
}

impl<'a> From<&'a str> for Pear<'a> {
    fn from(s: &'a str) -> Self {
        let ts = TokenStream::from(s);
        Self { ts }
    }
}
