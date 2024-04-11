use crate::lexer::{RPNIterator, Token, TokenIterator, TokenKind};
use std::collections::VecDeque;

pub(crate) struct TokenStream<'a> {
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
    buffer: VecDeque<Token>,
}

impl<'a> TokenStream<'a> {
    fn starts_with_let(&mut self) -> bool {
        let is_let = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            first_is_let
        };

        self.n_tokens_are(1, is_let)
    }

    fn is_decl(&mut self) -> bool {
        let is_ident = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            let second_is_ident = TokenStream::nth_token_is(1, index, token, TokenKind::Ident);
            let third_is_semi = TokenStream::nth_token_is(2, index, token, TokenKind::Semi);

            first_is_let || second_is_ident || third_is_semi
        };

        self.n_tokens_are(2, is_ident)
    }

    fn is_init(&mut self) -> bool {
        let is_init = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            let second_is_ident = TokenStream::nth_token_is(1, index, token, TokenKind::Ident);
            let third_is_equals = TokenStream::nth_token_is(2, index, token, TokenKind::Equals);

            first_is_let || second_is_ident || third_is_equals
        };

        self.n_tokens_are(3, is_init)
    }

    fn n_tokens_are(&mut self, n: usize, pred: fn(usize, &Token) -> bool) -> bool {
        let tokens = self.read(n);

        if tokens.len() < n {
            return false;
        }

        for (index, token) in tokens.iter().enumerate() {
            if index > (n - 1) {
                return true;
            }

            if !pred(index, token) {
                return false;
            }
        }

        true
    }

    fn nth_token_is(n: usize, index: usize, token: &Token, expected: TokenKind) -> bool {
        index == n && token.kind == expected
    }

    fn advance(&mut self, n: usize) {
        if n <= self.buffer.len() {
            return;
        }

        let to_take = n - self.buffer.len();

        for i in 0..to_take {
            if let Some(token) = self.tokens.next() {
                self.buffer.push_back(token);
            } else {
                // Early return for efficiency
                break;
            }
        }
    }

    pub(crate) fn read(&mut self, n: usize) -> VecDeque<Token> {
        self.advance(n);

        self.buffer.clone()
    }

    pub(crate) fn peek_next(&mut self) -> Option<Token> {
        self.read(1).pop_front()
    }

    pub(crate) fn peek_n(&mut self, n: usize) -> Option<Token> {
        self.read(n).pop_back()
    }
}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance(1);
        self.buffer.pop_front()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            tokens: Box::new(RPNIterator::from_iter(TokenIterator::from(s))),
            buffer: VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::lexer::TokenKind;
    use crate::tokenstream::TokenStream;

    #[test]
    fn iterator_complext_test() {
        let mut ts = TokenStream::from("let i = 10 + 20 + 30;");
        assert_eq!(
            ts.peek_next(),
            Some(Token {
                kind: TokenKind::Let,
                val: "let".to_string(),
                len: 3
            })
        );

        assert_eq!(
            ts.peek_n(2),
            Some(Token {
                kind: TokenKind::Ident,
                val: "i".to_string(),
                len: 1
            })
        );

        assert_eq!(
            ts.next(),
            Some(Token {
                kind: TokenKind::Let,
                val: "let".to_string(),
                len: 3
            })
        );

        assert_eq!(
            ts.next(),
            Some(Token {
                kind: TokenKind::Ident,
                val: "i".to_string(),
                len: 1
            })
        );
    }

    #[rustfmt::skip::macros(assert_eq)]
    #[test]
    fn reverse_polish_notation_iterator_test() {
        let mut ts = TokenStream::from("let i = 10 + 20 + 30;");
        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Let, val: "let".to_string(), len: 3 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Ident, val: "i".to_string(), len: 1 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Equals, val: "".to_string(), len: 1 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::NumericLiteral, val: "10".to_string(), len: 2 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::NumericLiteral, val: "20".to_string(), len: 2 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::NumericLiteral, val: "30".to_string(), len: 2 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Plus, val: "".to_string(), len: 1 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Plus, val: "".to_string(), len: 1 }));

        assert_eq!(ts.next(), Some(Token { kind: TokenKind::Semi, val: "".to_string(), len: 1 }));

        assert_eq!(ts.next(), None);
    }

    #[ignore] // TODO: Just for debugging
    #[test]
    fn reverse_polish_notation() {
        let mut ts = TokenStream::from("let a = 10 + 5 - 8;");
        let tokens = ts.next().unwrap();
    
        println!("{tokens:?}")
    }

    #[test]
    fn check_is_init() {
        let mut ts = TokenStream::from("let i = 10 + 20 + 30;");
        assert!(ts.is_init());
    }

    #[test]
    fn check_is_not_init() {
        let mut ts = TokenStream::from("let i;");
        assert!(!ts.is_init());
    }

    #[test]
    fn check_starts_with_let() {
        let mut ts = TokenStream::from("let a");
        assert!(ts.starts_with_let());
        assert!(!ts.is_init())
    }

    #[test]
    fn check_doesnt_start_with_let() {
        let mut ts = TokenStream::from("hello");
        assert!(!ts.starts_with_let());

        assert!(!ts.is_decl())
    }

    #[test]
    fn check_is_decl() {
        let mut ts = TokenStream::from("let a;");
        assert!(ts.is_decl());

        assert!(ts.starts_with_let());

        assert!(!ts.is_init());
    }

    #[test]
    fn check_is_not_decl() {
        let mut ts = TokenStream::from("a");
        assert!(!ts.is_decl());
    }

    #[test]
    fn read_n_from_token_stream() {
        let mut ts = TokenStream::from("a b c d;");

        let tokens = ts.read(2);
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens,
            vec![Token::try_from("a").unwrap(), Token::try_from("b").unwrap()]
        );

        let tokens = ts.read(4);
        assert_eq!(tokens.len(), 4);
        assert_eq!(
            tokens,
            vec![
                Token::try_from("a").unwrap(),
                Token::try_from("b").unwrap(),
                Token::try_from("c").unwrap(),
                Token::try_from("d").unwrap()
            ]
        );

        let tokens = ts.read(2);
        assert_eq!(tokens.len(), 4);
        assert_eq!(
            tokens,
            vec![
                Token::try_from("a").unwrap(),
                Token::try_from("b").unwrap(),
                Token::try_from("c").unwrap(),
                Token::try_from("d").unwrap()
            ]
        );

        let tokens = ts.read(5);
        assert_eq!(tokens.len(), 5);
        assert_eq!(
            tokens,
            vec![
                Token::try_from("a").unwrap(),
                Token::try_from("b").unwrap(),
                Token::try_from("c").unwrap(),
                Token::try_from("d").unwrap(),
                Token::try_from(";").unwrap()
            ]
        );

        let tokens = ts.read(0);
        assert_eq!(tokens.len(), 5);
        assert_eq!(
            tokens,
            vec![
                Token::try_from("a").unwrap(),
                Token::try_from("b").unwrap(),
                Token::try_from("c").unwrap(),
                Token::try_from("d").unwrap(),
                Token::try_from(";").unwrap()
            ]
        );

        let tokens = ts.read(100);
        assert_eq!(tokens.len(), 5);
        assert_eq!(
            tokens,
            vec![
                Token::try_from("a").unwrap(),
                Token::try_from("b").unwrap(),
                Token::try_from("c").unwrap(),
                Token::try_from("d").unwrap(),
                Token::try_from(";").unwrap()
            ]
        );
    }
}
