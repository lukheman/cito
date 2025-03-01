#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Print,
    IntLiteral,
    StringLiteral,
    Semicolon,
    LeftParent,
    RightParent,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub value: Option<String>,
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    current_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            current_index: 0,
        }
    }

    pub fn tokenization(&mut self) {
        let mut buf = String::new();

        while self.current_index < self.source.len() {
            let c: char = self.advance();

            match c {
                ';' => self.tokens.push(Token {
                    tokentype: TokenType::Semicolon,
                    value: None,
                }),
                '(' => self.tokens.push(Token {
                    tokentype: TokenType::LeftParent,
                    value: None,
                }),
                ')' => self.tokens.push(Token {
                    tokentype: TokenType::RightParent,
                    value: None,
                }),
                '"' => {
                    let mut value = String::new();

                    while self.peek().is_alphanumeric() || self.peek().is_whitespace() {
                        value.push(self.advance());
                    }

                    if self.peek() == '"' {
                        self.advance();
                        self.tokens.push(Token {
                            tokentype: TokenType::StringLiteral,
                            value: Some(value.clone()),
                        })
                    } else {
                        panic!("expected \"");
                    }
                }
                c => {
                    if c.is_alphabetic() {
                        buf.push(c);

                        while self.peek().is_alphabetic() {
                            buf.push(self.advance());
                        }

                        if buf == "cetak" {
                            self.tokens.push(Token {
                                tokentype: TokenType::Print,
                                value: None,
                            });
                            buf.clear();
                        }
                    } else if c.is_numeric() {
                        buf.push(c);

                        while self.peek().is_numeric() {
                            buf.push(self.advance());
                        }

                        self.tokens.push(Token {
                            tokentype: TokenType::IntLiteral,
                            value: Some(buf.clone()),
                        });

                        buf.clear();
                    }
                }
            }
        }
    }

    fn peek(&self) -> char {
        if !self.current_index < self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current_index).unwrap();
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current_index).unwrap();
        if !self.current_index < self.source.len() {
            return '\0';
        }

        self.current_index += 1;
        return c;
    }
}
