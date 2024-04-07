use crate::lexer::{tokenize, Token, TokenKind};

pub(crate) struct TokenStream<'a> {
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
    buffer: Vec<Token>,
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

    fn take(&mut self, n: usize) {
        if n <= self.buffer.len() {
            return;
        }

        let to_take = n - self.buffer.len();

        for i in 0..to_take {
            if let Some(token) = self.tokens.next() {
                self.buffer.push(token);
            } else {
                // Early return for efficiency
                break;
            }
        }
    }

    pub(crate) fn read(&mut self, n: usize) -> Vec<Token> {
        self.take(n);

        self.buffer.clone()
    }

    fn peek_next(&mut self) -> Option<Token> {
        self.read(1).pop()
    }
}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.take(1);
        self.buffer.pop()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            tokens: Box::new(tokenize(s)),
            buffer: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

impl Ident {
    fn parse(ts: &mut TokenStream) -> Result<Self, String> {
        Err("Expected Identifier, got... something else. TODO: better msg?".to_string())
    }
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn parse(ts: TokenStream) -> Result<Self, String> {
        todo!()
    }
}

struct Block {
    stmts: Vec<Stmt>,
}

enum Local {
    Decl { ident: Ident },
    Init { ident: Ident, val: Box<Expr> },
}

enum Stmt {
    Local(Box<Local>),
    Expr(Box<Expr>),
    Semi(Box<Expr>),
    Empty,
}

struct Fn {
    ident: Ident,
    block: Block,
}

enum LitKind {
    String,
    Number,
}

enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Block(Box<Block>),
    Lit(LitKind),
    FnCall(Fn, Vec<Ident>),
}

impl Expr {
    fn parse(ts: &mut TokenStream) -> Self {
        todo!()
    }
}

struct Mod {
    entry: Option<Fn>,
    functions: Vec<Fn>,
}

impl Mod {
    fn parse(ts: TokenStream) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::parser::TokenStream;

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
