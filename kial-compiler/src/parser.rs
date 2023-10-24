use crate::lexer::{tokenize, Token, TokenKind};

struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    fn starts_with_let(&self) -> bool {
        let is_let = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            first_is_let
        };

        self.n_tokens_are(1, is_let)
    }

    fn is_decl(&self) -> bool {
        let is_ident = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            let second_is_ident = TokenStream::nth_token_is(1, index, token, TokenKind::Ident);
            let third_is_semi = TokenStream::nth_token_is(2, index, token, TokenKind::Semi);

            first_is_let || second_is_ident || third_is_semi
        };

        self.n_tokens_are(2, is_ident)
    }

    fn is_init(&self) -> bool {
        let is_init = |index: usize, token: &Token| -> bool {
            let first_is_let = TokenStream::nth_token_is(0, index, token, TokenKind::Let);
            let second_is_ident = TokenStream::nth_token_is(1, index, token, TokenKind::Ident);
            let third_is_equals = TokenStream::nth_token_is(2, index, token, TokenKind::Equals);

            first_is_let || second_is_ident || third_is_equals
        };

        self.n_tokens_are(3, is_init)
    }

    fn n_tokens_are(&self, n: usize, pred: fn(usize, &Token) -> bool) -> bool {
        for (index, token) in self.tokens.iter().enumerate() {
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
}

impl From<&str> for TokenStream {
    fn from(s: &str) -> Self {
        Self {
            tokens: tokenize(s).collect(),
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
    use crate::parser::TokenStream;

    #[test]
    fn check_is_init() {
        let ts = TokenStream::from("let i = 10 + 20 + 30;");
        assert!(ts.is_init());
    }

    #[test]
    fn check_is_not_init() {
        let ts = TokenStream::from("let i;");
        assert!(!ts.is_init());
    }

    #[test]
    fn check_starts_with_let() {
        let ts = TokenStream::from("let a");
        assert!(ts.starts_with_let());
    }

    #[test]
    fn check_doesnt_start_with_let() {
        let ts = TokenStream::from("hello");
        assert!(!ts.starts_with_let());
    }

    #[test]
    fn check_is_decl() {
        let ts = TokenStream::from("let a;");
        assert!(ts.is_decl());
    }

    #[test]
    fn check_is_not_decl() {
        let ts = TokenStream::from("a");
        assert!(!ts.is_decl());
    }
}
