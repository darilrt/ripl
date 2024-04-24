use crate::token::{Location, Token, TokenKind};

pub struct Lexer {
    input: String,
    pos: usize,
    location: Location,
}

const KEYWORDS: [&str; 1] = ["let"];

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            pos: 0,
            location: Location { line: 1, column: 1 },
        }
    }

    pub fn read_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token.kind == TokenKind::Eof || token.kind == TokenKind::Unknown {
                break;
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let c = self.peek();

        if c.is_none() {
            return Token {
                kind: TokenKind::Eof,
                lit: "".to_string(),
                loc: self.location,
            };
        }

        let c = c.unwrap();

        if c.is_alphabetic() || c == '_' {
            return self.read_ident();
        }

        if c.is_digit(10) {
            return self.read_number();
        }

        if c == '"' || c == '\'' {
            return self.read_string(c);
        }

        if c == '/' && self.peek_next() == Some('/') {
            while let Some(c) = self.next() {
                if c == '\n' {
                    break;
                }
            }
            return self.next_token();
        }

        self.read_unknown()
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.pos);
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            } else {
                self.location.column += 1;
            }
            return Some(c);
        }
        None
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn peek_next(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn read_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.next();
            } else {
                break;
            }
        }

        Token {
            kind: if KEYWORDS.contains(&&ident[..]) {
                TokenKind::Keyword
            } else {
                TokenKind::Ident
            },
            lit: ident,
            loc: self.location,
        }
    }

    pub fn read_unknown(&mut self) -> Token {
        let c = self.next().unwrap();

        const SYMBOLS: [char; 13] = [
            '(', ')', '{', '}', ',', ';', ':', '.', '[', ']', '@', '$', '?',
        ];
        const OPERATORS: [char; 10] = ['+', '-', '*', '/', '=', '!', '<', '>', '&', '|'];

        if OPERATORS.contains(&c) && self.peek() == Some('=') {
            return self.doble_symbol(&format!("{}=", c));
        } else if OPERATORS.contains(&c) {
            return Token {
                kind: TokenKind::Operator,
                lit: c.to_string(),
                loc: self.location,
            };
        }

        if c == '-' && self.peek() == Some('>') {
            return self.doble_symbol("->");
        }

        if SYMBOLS.contains(&c) {
            return Token {
                kind: TokenKind::Symbol,
                lit: c.to_string(),
                loc: self.location,
            };
        }

        Token {
            kind: TokenKind::Unknown,
            lit: c.to_string(),
            loc: self.location,
        }
    }

    pub fn doble_symbol(&mut self, sym: &str) -> Token {
        self.next();
        self.next();
        Token {
            kind: TokenKind::Operator,
            lit: sym.to_owned(),
            loc: self.location,
        }
    }

    pub fn read_number(&mut self) -> Token {
        let mut number = String::new();

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                number.push(c);
                self.next();
            } else {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c == '.' {
                number.push(c);
                self.next();
                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        self.next();
                    } else {
                        break;
                    }
                }

                return Token {
                    kind: TokenKind::Float,
                    lit: number,
                    loc: self.location,
                };
            }
        }

        Token {
            kind: TokenKind::Integer,
            lit: number,
            loc: self.location,
        }
    }

    pub fn read_string(&mut self, quote: char) -> Token {
        let mut string = String::new();
        self.next();

        while let Some(c) = self.peek() {
            if c == quote {
                break;
            }
            string.push(c);
            self.next();
        }

        self.next();

        Token {
            kind: TokenKind::String,
            lit: string,
            loc: self.location,
        }
    }
}
