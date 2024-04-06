#![allow(unused)]

use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::Chars;

use self::TokenKind::*;

const EOF_CHAR: char = '\0';

fn is_valid_id_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

struct Cursor<'a> {
    chars: Chars<'a>,
    len: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Cursor<'a> {
        Self {
            chars: input.chars(),
            len: input.len(),
        }
    }

    fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }
    fn shrink_1(&mut self) {
        self.len -= 1;
    }

    fn pos(&self) -> usize {
        self.len - self.chars.as_str().len()
    }

    fn pos_reset(&mut self) {
        self.len = self.chars.as_str().len();
    }

    fn take_1(&mut self) -> char {
        let c = self.chars.next().unwrap_or(EOF_CHAR);

        c
    }

    fn take_while(&mut self, pred: impl Fn(char) -> bool) -> String {
        let mut s = vec![];
        while pred(self.first()) {
            s.push(self.take_1());
        }

        s.iter().collect()
    }

    fn extract_ident(&mut self) -> String {
        self.take_while(|c| c.is_alphanumeric())
    }

    fn extract_num(&mut self) -> String {
        self.take_while(|c| c.is_numeric())
    }

    fn extract_double_quoted_string(&mut self) -> String {
        let mut str = vec![
            self.take_while(|c| c == '"'),
            self.take_while(|c| c != '"'),
            self.take_while(|c| c == '"'),
        ];

        str.concat()
    }

    fn advance_token(&mut self) -> Token {
        let mut first_char = EOF_CHAR;
        while let c = self.take_1() {
            first_char = c;
            if !c.is_ascii_whitespace() {
                break;
            }

            self.pos_reset();
        }

        let mut val = String::new();
        let token_kind = match first_char {
            EOF_CHAR => Eof,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '%' => Percent,
            '=' => Equals,
            ';' => Semi,
            '"' => {
                let str = format!("{}{}", first_char, self.extract_double_quoted_string());
                val = str;
                StringLiteral
            }

            c if c.is_ascii_whitespace() => Whitespace,

            c @ '0'..='9' => {
                let num = format!("{}{}", first_char, self.extract_num());
                val = num;
                NumericLiteral
            }

            c if is_valid_id_start(c) => {
                let ident = format!("{}{}", first_char, self.extract_ident());
                let kind = match ident.as_str() {
                    "let" => Let,
                    "func" => Func,
                    _ => Ident,
                };

                val = ident;
                kind
            }

            _ => Unknown,
        };

        let token = Token {
            kind: token_kind,
            val,
            len: self.pos(),
        };

        self.pos_reset();

        token
    }
}

#[derive(Debug, PartialEq)]
struct Val {
    val: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub val: String,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, val: String, len: usize) -> Self {
        Self { kind, val, len }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: Eof,
            val: String::new(),
            len: 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let repr = match self.kind {
            Let => "let".to_string(),
            Func => "func".to_string(),
            Ident => self.val.clone(),
            StringLiteral => self.val.clone(),
            NumericLiteral => self.val.clone(),
            OpenParen => "(".to_string(),
            CloseParen => ")".to_string(),
            OpenBrace => "{".to_string(),
            CloseBrace => "}".to_string(),
            OpenBracket => "[".to_string(),
            CloseBracket => "]".to_string(),
            Semi => ";".to_string(),
            Equals => "=".to_string(),
            Plus => "+".to_string(),
            Minus => "-".to_string(),
            Star => "*".to_string(),
            Slash => "/".to_string(),
            Percent => "%".to_string(),
            Whitespace => "WHITESPACE".to_string(),
            Eof => "EOF".to_string(),
            Unknown => "UNKNOWN".to_string(),
        };

        repr.fmt(f) // Fixes weird alignment https://stackoverflow.com/a/77937993
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Ident,          // function & variable names
    StringLiteral,  // String literals
    NumericLiteral, // Numeric literals
    OpenParen,      // (
    CloseParen,     // )
    OpenBrace,      // {
    CloseBrace,     // }
    OpenBracket,    // [
    CloseBracket,   // ]
    Semi,           // ;
    Equals,         // =
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Let,            // let
    Func,           // func
    Percent,
    Whitespace,
    Eof,
    Unknown,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind == Eof {
            None
        } else {
            Some(token)
        }
    })
}

fn expr_to_postfix_notation2<'a>(
    iter: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Token> + 'a {
    std::iter::from_fn(move || Some(Token::default()))
}

fn expr_to_postfix_notation<'a>(
    mut iter: impl Iterator<Item = Token> + 'a,
) -> impl Iterator<Item = Token> + 'a {
    struct RPNCursor<'a> {
        rpn: VecDeque<Token>,
        stack: Vec<Token>,
        iter: Box<dyn Iterator<Item = Token> + 'a>,
    }

    impl RPNCursor<'_> {
        fn print_debug(&self) {
            print!("rpn: ");
            for x in &self.rpn {
                print!("{x} ");
            }
            print!("{:10}", " ");

            print!("stack: ");
            for x in &self.stack {
                print!("{x} ");
            }
            println!();
        }

        fn precedence(token: &Token) -> u8 {
            // TODO: This can probably be a trait
            match token.kind {
                OpenParen => 3,
                CloseParen => 3,
                Star => 2,
                Slash => 2,
                Percent => 2,
                Plus => 1,
                Minus => 1,
                _ => unreachable!(
                    "This TokenKind \"{:?}\" does not have operator precedence.",
                    token.kind
                ),
            }
        }

        fn handle_operand(&mut self, token: Token) {
            self.rpn.push_back(token);
        }

        fn handle_operator(&mut self, token: Token) {
            let precedence_of_token = Self::precedence(&token);

            loop {
                // TODO: Handle Parenthesis: https://www.youtube.com/watch?v=QxHRM0EQHiQ
                let Some(last) = self.stack.last() else {
                    self.stack.push(token);
                    return;
                };

                let precedence_of_last = Self::precedence(last);
                if precedence_of_last <= precedence_of_token {
                    self.stack.push(token);
                    return;
                }

                if precedence_of_last > precedence_of_token {
                    let last = self.stack.pop().unwrap();
                    self.rpn.push_back(last);
                }
            }
        }

        // TODO: This needs to be a trait
        fn next(&mut self) -> Option<Token> {
            while let Some(token) = self.iter.next() {
                print!("{token:2}{:4}| ", " ");
                self.print_debug();
                match token.kind {
                    NumericLiteral => self.handle_operand(token),
                    OpenParen | CloseParen | Equals | Plus | Minus | Star | Slash | Percent => {
                        self.handle_operator(token)
                    }
                    _ => unreachable!("This shouldn't be called with token: {:?}", token),
                };

                if !self.rpn.is_empty() {
                    return self.rpn.pop_front();
                }
            }

            if !self.stack.is_empty() {
                while let Some(top) = self.stack.pop() {
                    self.rpn.push_back(top)
                }
            }

            self.rpn.pop_front()
        }
    }

    let mut cursor = RPNCursor {
        rpn: VecDeque::new(),
        stack: vec![],
        iter: Box::new(iter),
    };

    std::iter::from_fn(move || cursor.next())
}

#[rustfmt::skip::macros(assert_eq)]
#[cfg(test)]
mod tests {
    use crate::lexer::TokenKind::*;
    use crate::lexer::{expr_to_postfix_notation, tokenize, Token, TokenKind};

    #[test]
    fn simple_expr_to_postfix_notations() {
        let s = "10 + 20 * 5 - 15 / 3 * 6 + 4";
        // 10 20 5 * 15 3 6 * / 4 + - +
        // 952+-3*
        // 6

        let token_iter = tokenize(s);
        let mut rpn_iter = expr_to_postfix_notation(token_iter);
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"10".to_string(), 2)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"20".to_string(), 2)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"5".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Star, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"15".to_string(), 2)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"3".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"6".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Star, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Slash, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(NumericLiteral,"4".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Minus, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));
        assert_eq!(rpn_iter.next(), None);
    }

    #[test]
    fn tokenize_simple_func() {
        let text = "func main() {}";
        let mut token_iter = tokenize(text);

        assert_eq!(token_iter.next(), Some(Token { kind: Func, val: "func".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "main".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenParen, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseParen, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenBrace, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseBrace, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_simple_program() {
        let text =
            "func main() {\nlet word1 = \"hello\";\nlet word2 = \" world!\";\nword1 + word2\n}";
        let mut token_iter = tokenize(text);

        // func main() {\n
        assert_eq!(token_iter.next(), Some(Token { kind: Func, val: "func".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "main".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenParen, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseParen, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenBrace, val: "".to_string(), len: 1 }));

        // let word1 = "hello";\n
        assert_eq!(token_iter.next(), Some(Token { kind: Let, val: "let".to_string(), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "word1".to_string(), len: 5 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: StringLiteral, val: r#""hello""#.to_string(), len: 7 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: "".to_string(), len: 1 }));

        // let word2 = " world!";\n
        assert_eq!(token_iter.next(), Some(Token { kind: Let, val: "let".to_string(), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "word2".to_string(), len: 5}));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: StringLiteral, val: r#"" world!""#.to_string(), len: 9 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: "".to_string(), len: 1 }));

        // word1 + word2\n
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "word1".to_string(), len: 5 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Plus, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "word2".to_string(), len: 5 }));

        // }
        assert_eq!(token_iter.next(), Some(Token { kind: CloseBrace, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_literal_num() {
        let s = "987654321 ";
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token { kind: NumericLiteral, val: "987654321".to_string(), len: 9 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_literal_string() {
        let s = r#"let text = "hello world";"#;
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token { kind: Let, val: "let".to_string(), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "text".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: StringLiteral, val: r#""hello world""#.to_string(), len: 13 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: "".to_string(), len: 1 }));

        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_expression() {
        let s = "10 + 20 * 5 - 15 / 3 * 6 + 4";
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "10".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "20".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Star, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "5".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Minus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "15".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Slash, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "3".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Star, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "6".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(NumericLiteral, "4".to_string(), 1)));

        assert_eq!(token_iter.next(), None);
    }
}
