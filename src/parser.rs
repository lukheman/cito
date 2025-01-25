use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub enum NodeExpr {
    NodeExprStringLiteral { token: Token },
    NodeExprIntLiteral { token: Token },
}

#[derive(Debug)]
pub enum NodeStmt {
    NodePrint { expr: NodeExpr },
}

#[derive(Debug)]
pub struct NodeProgram {
    pub statements: Vec<NodeStmt>,
}

impl NodeProgram {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    fn parse_expr(&mut self) -> Option<NodeExpr> {
        if !self.peek().is_none() {
            let token = self.advance().unwrap();

            if token.tokentype == TokenType::StringLiteral {
                return Some(NodeExpr::NodeExprStringLiteral { token });
            } else if token.tokentype == TokenType::IntLiteral {
                return Some(NodeExpr::NodeExprIntLiteral { token });
            }
        }

        return None;
    }

    pub fn parse(&mut self) -> Option<NodeProgram> {
        let mut nodeprogram = NodeProgram::new();

        while !self.peek().is_none() {
            let token = self.advance().unwrap();

            if token.tokentype == TokenType::Print {
                if !self.peek().is_none() && self.peek().unwrap().tokentype == TokenType::LeftParent
                {
                    self.advance();
                    if let Some(expr) = self.parse_expr() {
                        // dbg!(&expr);
                        if !self.peek().is_none()
                            && self.peek().unwrap().tokentype == TokenType::RightParent
                        {
                            self.advance();
                            if !self.peek().is_none()
                                && self.peek().unwrap().tokentype == TokenType::Semicolon
                            {
                                nodeprogram.statements.push(NodeStmt::NodePrint { expr });
                            }
                        } else {
                            panic!("expected )");
                        }
                    } else {
                        panic!("expected expr");
                    }
                } else {
                    panic!("expected (");
                }
            }
        }

        return Some(nodeprogram);
    }

    fn peek(&self) -> Option<Token> {
        if self.current_index >= self.tokens.len() {
            return None;
        }

        return Some(self.tokens[self.current_index].clone());
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current_index >= self.tokens.len() {
            return None;
        }
        let token = &self.tokens[self.current_index];

        self.current_index += 1;
        return Some(token.clone());
    }
}
