use crate::literal::Literal;

use crate::error::LoxResult;
use crate::expr::*;
use crate::stmt::*;
use crate::token::*;
use crate::token_type::*;

pub struct Parser<'a> {
    pub tokens: &'a [Token],
    current: usize,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }
    pub fn success(&self) -> bool {
        !self.had_error
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
        let result = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }
        result
    }
    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;
        let initializer = if self.is_match(&[TokenType::Assign]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;

        Ok(Stmt::Var(StmtVar { name, initializer }))
    }

    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.is_match(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.is_match(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.is_match(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(StmtBlock {
                statements: self.block()?,
            }));
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for' keyword")?;
        let initializer = if self.is_match(&[TokenType::Semicolon]) {
            None
        } else if self.is_match(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let condition = if self.is_match(&[TokenType::Semicolon]) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block(StmtBlock {
                statements: vec![body, Stmt::Expression(StmtExpression { expression: incr })],
            });
        }

        body = Stmt::While(StmtWhile {
            condition: if let Some(cond) = condition {
                cond
            } else {
                Expr::Literal(ExprLiteral {
                    value: Some(Literal::Boolean(true)),
                })
            },
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = Stmt::Block(StmtBlock {
                statements: vec![init, body],
            })
        }

        Ok(body)
    }
    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while' statement")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition in 'while' statement",
        )?;

        let body = self.statement()?;

        Ok(Stmt::While(StmtWhile {
            condition,
            body: Box::new(body),
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after the statement!")?;
        Ok(Stmt::Print(StmtPrint { expression: value }))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if' statement!")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition expression!",
        )?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.is_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(StmtIf {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after the statement!")?;
        Ok(Stmt::Expression(StmtExpression { expression: expr }))
    }

    pub fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Assign]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(ExprAssign {
                    name: expr.name.clone(),
                    value: Box::new(value),
                }));
            }
            self.error(&equals, "Invalid l-value for assignment");
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(ExprLogical {
                left: Box::new(expr),
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical(ExprLogical {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxResult> {
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

    fn comparison(&mut self) -> Result<Expr, LoxResult> {
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
    fn term(&mut self) -> Result<Expr, LoxResult> {
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
    fn factor(&mut self) -> Result<Expr, LoxResult> {
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

    fn unary(&mut self) -> Result<Expr, LoxResult> {
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

    fn primary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&[TokenType::String, TokenType::Number, TokenType::Constant]) {
            let tok = self.previous();
            if let Some(value) = &tok.literal {
                return Ok(Expr::Literal(ExprLiteral {
                    value: Some(value.clone()),
                }));
            }
        }
        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(ExprVariable {
                name: self.previous().clone(),
            }));
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(ExprGrouping {
                expression: Box::new(expr),
            }));
        }

        let token = self.peek().clone();
        Err(self.error(&token, "Failed when parsing primary tokens"))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxResult> {
        if self.check(ttype) {
            Ok(self.advance().clone())
        } else {
            let token = self.peek().clone();
            Err(self.error(&token, message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> LoxResult {
        self.had_error = true;
        LoxResult::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }
            if matches!(
                self.peek().token_type(),
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
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
