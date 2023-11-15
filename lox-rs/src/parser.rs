use crate::error::LoxError;
use crate::expr::*;
use crate::token::*;
use crate::token_type::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

#[cfg(test)]
fn test_simple_token_sequence() {
    let tokens = vec![
        Token::new(TokenType::LeftBrace, "(".to_string(), 0, None)];
    let mut  parser = Parser::new(tokens);
    let result = parser.expression();

}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::Equals]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(ExprBinary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;
        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary(ExprBinary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(ExprBinary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(ExprBinary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(ExprUnary {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Identifier]) {
            let tok = self.previous();
            if let Some(value) = &tok.literal {
                return Ok(Expr::Literal(ExprLiteral {
                    value: Some(value.clone()),
                }));
            }
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression".to_string(),
            )?;
            return Ok(Expr::Grouping(ExprGrouping {
                expression: Box::new(expr),
            }));
        }
        Err(LoxError::parse_error(
            self.peek(),
            "failed parsing primary tokens".to_string(),
        ))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(ttype) {
            Ok(self.advance().clone())
        } else {
            Err(Parser::error(self.peek(), message))
        }
    }

    fn error(token: &Token, message: String) -> LoxError {
        LoxError::parse_error(token, message )
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is(ttype)
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
