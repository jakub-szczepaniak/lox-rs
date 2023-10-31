use std::fmt;
pub struct LoxError {
    pub message: String,
    pub line: usize,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Error: {}", self.line, self.message)
    }
}
impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        LoxError { message, line }
    }
    pub fn report(&self, loc: String) {
        println!("[{}:{}] Error: {}", self.line, loc, self.message);
    }    
}

