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
        println!("{} : {}", self.len, self.chars.as_str().len());
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
        let first_char = self.take_1();

        let mut val = None;
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
                val = Some(str);
                Literal
            }

            c if c.is_ascii_whitespace() => Whitespace,

            c @ '0'..='9' => {
                let mut num = format!("{}{}", first_char, self.extract_num());
                val = Some(num);
                Literal
            }

            c if is_valid_id_start(c) => {
                let ident = format!("{}{}", first_char, self.extract_ident());
                val = Some(ident);
                Ident
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

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) val: Option<String>,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
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
    Percent,
    Whitespace,
    Eof,
    Unknown,
}

pub(crate) fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
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

#[rustfmt::skip::macros(assert_eq)]
#[cfg(test)]
mod tests {
    use crate::lexer::TokenKind::*;
    use crate::lexer::{tokenize, Token, Val};

    #[test]
    fn tokenize_simple_func() {
        let text = "func main() {}";
        let mut token_iter = tokenize(text);

        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("func".to_string()), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("main".to_string()), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenParen, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseParen, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenBrace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseBrace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_simple_program() {
        let text =
            "func main() {\nlet word1 = \"hello\";\nlet word2 = \" world!\";\nword1 + word2\n}";
        let mut token_iter = tokenize(text);

        // func main() {\n
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("func".to_string()), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("main".to_string()), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenParen, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: CloseParen, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: OpenBrace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));

        // let word1 = "hello";\n
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("let".to_string()), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("word1".to_string()), len: 5 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: Some(r#""hello""#.to_string()), len: 7 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));

        // let word2 = " world!";\n
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("let".to_string()), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("word2".to_string()), len: 5}));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: Some(r#"" world!""#.to_string()), len: 9 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));

        // word1 + word2\n
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("word1".to_string()), len: 5 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Plus, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("word2".to_string()), len: 5 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));

        // }
        assert_eq!(token_iter.next(), Some(Token { kind: CloseBrace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_literal_num() {
        let s = "987654321 ";
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: Some("987654321".to_string() ), len: 9 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn tokenize_literal_string() {
        let s = r#"let text = "hello world";"#;
        let mut token_iter = tokenize(s);

        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("let".to_string()), len: 3 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Ident, val: Some("text".to_string()), len: 4 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Equals, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Whitespace, val: None, len: 1 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Literal, val: Some(r#""hello world""#.to_string()), len: 13 }));
        assert_eq!(token_iter.next(), Some(Token { kind: Semi, val: None, len: 1 }));

        assert_eq!(token_iter.next(), None);
    }
}
