use crate::error::LoxError;
use crate::token_type::TokenType;
use crate::token::Token;
use crate::token::Literal;
pub struct Scanner {
    source: Vec<char>,
    tokens:  Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source : source.chars().collect(),
            tokens : Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
   } 
   pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(e) => {
                    e.report(self.current.to_string());
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));
        if let Some(e) = had_error {
            return Err(e);
        } 
        Ok(&self.tokens)
   }
    fn is_at_end(&self) -> bool {
       self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => { 
                if self.take_expected('=') {
                     self.add_token(TokenType::BangEqual) 
                } else { self.add_token(TokenType::Bang) } },
            '=' => { 
                if self.take_expected('=') {
                     self.add_token(TokenType::Equals) 
                } else { self.add_token(TokenType::Assign) } },
            '<' => {
                if self.take_expected('=') {
                    self.add_token(TokenType::LessEqual)
                } else { self.add_token(TokenType::Less) } },
            '>' => {
                if self.take_expected('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else { self.add_token(TokenType::Greater) } },
            '/' => {
                if self.take_expected('/') {
                    while let Some(ch) = self.peek() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            _ => if c.is_ascii_alphabetic() || c == '_' {
                self.identifier()
            }
            _ => return Err(LoxError::error(self.line, "Unexpected character".to_string()))
        }
        Ok(())
    }
    
    
    fn advance(&mut self) -> char {
       let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }
    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }
    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, lexeme, self.line, literal));  
    }
    fn take_expected(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(c) if *c == expected => {
                self.current += 1;
                true
            }
            _ => false
        }
    }
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current+1).copied()
    }
    
    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text : String = self.source[self.start..self.current].iter().collect();
        self.add_token_object(TokenType::Identifier, Some(Literal::Identifier(text)))
    }

    fn is_alpha_numeric(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_ascii_alphanumeric(),
            None => false}
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }               
          
        let value : String = self.source[self.start..self.current].iter().collect();
        self.add_token_object(TokenType::Number, Some(Literal::Number(value.parse::<f64>().unwrap())));
        Ok(())
    }

    fn is_digit(ch: Option<char>) -> bool {
        match ch {
            Some(ch) => ch.is_ascii_digit(),
            None => false
        }
    }
    fn string(&mut self) -> Result<(), LoxError> { 
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => ()
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxError::error(self.line, "Unterminated string".to_string()));
        }
        
        self.advance();
        let value = self.source[self.start+1..self.current-1].iter().collect();
        self.add_token_object(TokenType::String, Some(Literal::String(value)));
        Ok(())
    }

}