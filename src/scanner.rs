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
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::eof(self.line));
     
     Ok(&self.tokens)
   }
    fn is_at_end(&self) -> bool {
       self.current >= self.source.len()
    }
    fn scan_token(&mut self) {
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => unreachable!("Unexpected character: {}", c)
        }
    
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
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            unreachable!("Unterminated string");
        }
        self.advance();
        let value = self.source[self.start+1..self.current-1].iter().collect();
        self.add_token_object(TokenType::String, Some(Literal::String(value)));
    }

}