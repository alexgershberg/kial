use std::fmt::{Debug, Display, Formatter};
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
                Literal
            }

            c if c.is_ascii_whitespace() => Whitespace,

            c @ '0'..='9' => {
                let num = format!("{}{}", first_char, self.extract_num());
                val = num;
                Literal
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self.kind {
            Let => "let".to_string(),
            Func => "func".to_string(),
            Ident => self.val.clone(),
            Literal => self.val.clone(),
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

        f.write_str(repr.as_str())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Ident,        // function & variable names
    Literal,      // Numbers, string literals
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Semi,         // ;
    Equals,       // =
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Let,          // let
    Func,         // func
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

fn expr_to_postfix_notation<'a>(
    iter: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Token> + 'a {
    std::iter::from_fn(move || Some(Token::default()))
}

#[rustfmt::skip::macros(assert_eq)]
#[cfg(test)]
mod tests {
    use crate::lexer::TokenKind::*;
    use crate::lexer::{expr_to_postfix_notation, tokenize, Token};

    #[test]
    fn simple_expr_to_postfix_notations() {
        let s = "10 + 20 * 5 - 15 / 3 * 6 + 4";
        // 10 20 5 * + 4 15 3 6 * / + -
        // 952+-3*
        // 6

        let token_iter = tokenize(s);
        expr_to_postfix_notation(token_iter);
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
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: r#""hello""#.to_string(), len: 7 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: "".to_string(), len: 1 }));

        // let word2 = " world!";\n
        assert_eq!(token_iter.next(), Some(Token { kind: Let, val: "let".to_string(), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "word2".to_string(), len: 5}));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: r#"" world!""#.to_string(), len: 9 }));
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

        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: "987654321".to_string(), len: 9 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_literal_string() {
        let s = r#"let text = "hello world";"#;
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token { kind: Let, val: "let".to_string(), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: "text".to_string(), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: "".to_string(), len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: r#""hello world""#.to_string(), len: 13 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: "".to_string(), len: 1 }));

        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_expression() {
        let s = "10 + 20 * 5 - 15 / 3 * 6 + 4";
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "10".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "20".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Star, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "5".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Minus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "15".to_string(), 2)));
        assert_eq!(token_iter.next(), Some(Token::new(Slash, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "3".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Star, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "6".to_string(), 1)));
        assert_eq!(token_iter.next(), Some(Token::new(Plus, "".to_string(), 1)));

        assert_eq!(token_iter.next(), Some(Token::new(Literal, "4".to_string(), 1)));

        assert_eq!(token_iter.next(), None);
    }
}
