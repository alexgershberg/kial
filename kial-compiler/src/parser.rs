use crate::lexer;
use crate::lexer::{Token, TokenKind};

struct TokenStream<'a> {
    tokens: Box<dyn Iterator<Item = Token> + 'a>,
}

impl<'a> TokenStream<'a> {
    fn expect(&mut self, token_kind: TokenKind) -> bool {
        let mut p = self.peekable();
        if let Some(token) = p.peek() {
            token.kind == token_kind
        } else {
            false
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            tokens: Box::new(lexer::tokenize(value)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ident(String);

impl Ident {
    fn parse(ts: &mut TokenStream) -> Result<Self, String> {
        if let Some(token) = ts.next() {
            return Ok(Self(token.val.unwrap()));
        }

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
    use crate::parser::{Expr, Ident, TokenStream};

    #[test]
    fn parse_binary_expr() {
        let mut ts = TokenStream::from("HelloWorld");
        // Expr::parse(&mut ts);
        assert_eq!(Ident::parse(&mut ts), Ok(Ident("HelloWorld".to_string())))
    }
}

/*

Roman num -> D

A -> ( 0 | 4 | 5 + ( 1 | 2 | 3 | 0 ) ) | 9 // Max 9
B -> ( 0 A | 10 A | 20 A | 30 A | 40 A | 50 A | 60 A | 70 A | 80 A | 90 A ) // 99
C -> ( 100 B | 200 B | 300 B | 400 B | 500 B | 600 B | 700 B | 800 B | 900 B ) // 999
D -> ( 1000 C | 2000 C | 3000 C | 4000 C) // 4999


units = I | II | III | Empty // 0, 1, 2, 3

         0      4      9
q -> ( Empty | IV | V units ) // 0..=9

tens = X | XX | XXX | Empty // 0, 10, 20, 30

         39      49       89       99
w -> ( tens q | LX q | L tens q | XC q ) // 0..99
          100  200  300
hundreds = C | CC | CCC | Empty // 0, 100, 200, 300

         399        499       899         999
e = ( hundreds w | CD w | D hundreds w |       )
r



I 1
II 2
III 3
IV 4
V 5
VI 6
VII 7
VIII 8
IX 9
X 10
XXX
XL 40
L 50
LX 60
LXX 70
LXXX 80

XC 90
C 100
CX 110

CD 400
D 500
DC 600
DCC 700
DCCC 800
M 1000





*/
