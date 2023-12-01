use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::{error::LoxError, literal::Literal, token::Token};
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, var_name: &str, value: Literal) {
        self.values.insert(var_name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, LoxError> {
        if let Some(value) = self.values.get(token.as_string()) {
            return Ok(value.clone());
        }
        Err(LoxError::interp_error(
            &token.clone(),
            &format!("Undefined variable: {}", token.as_string()),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<(), LoxError> {
        if let Entry::Occupied(mut entry) = self.values.entry(name.as_string().to_string()) {
            entry.insert(value);
            Ok(())
        } else {
            Err(LoxError::interp_error(
                name,
                &format!("Undefined variable: {}", name.as_string()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::token_type::TokenType;
    fn make_key_value(key: &str, value: Literal) -> (&str, Literal) {
        (key, value)
    }

    #[test]
    fn test_can_reassign_existing_variable() {
        let mut env = Environment::new();
        let var_name = Token::new(TokenType::Identifier, "my_var".to_string(), 0, None);

        env.define("my_var", Literal::Number(42.0));
        assert!(env.assign(&var_name, Literal::Number(24.0)).is_ok());
        assert_eq!(env.get(&var_name).unwrap(), Literal::Number(24.0));
    }
    #[test]
    fn test_returns_error_for_unknown_variable() {
        let mut env = Environment::new();
        let var_name = Token::new(TokenType::Identifier, "no_var".to_string(), 0, None);
        assert!(env.assign(&var_name, Literal::Boolean(false)).is_err());
    }

    #[test]
    fn test_env_can_define_variable() {
        let mut env = Environment::new();

        let key_val = make_key_value("my_bool", Literal::Boolean(true));

        env.define(key_val.0, key_val.1);

        assert!(env.values.contains_key(&"my_bool".to_string()));

        let token = Token::new(TokenType::Identifier, "my_bool".to_string(), 0, None);

        let val = env.get(&token).unwrap();

        assert_eq!(val, Literal::Boolean(true));
    }
    #[test]
    fn test_can_get_value_of_variable() {
        let mut env = Environment::new();
        let key_val = make_key_value("my_var", Literal::Number(42.9));

        env.define(key_val.0, key_val.1);

        let identifier = Token::new(TokenType::Identifier, "my_var".to_string(), 0, None);

        let result = env.get(&identifier).unwrap();

        assert_eq!(result, Literal::Number(42.9));
    }

    #[test]
    fn test_can_redefine_variable() {
        let mut env = Environment::new();

        let key_val_1 = make_key_value("my_var", Literal::Boolean(true));
        let key_val_2 = make_key_value("my_var", Literal::Boolean(false));

        let identifier = Token::new(TokenType::Identifier, "my_var".to_string(), 0, None);
        env.define(key_val_1.0, key_val_1.1);

        let val = env.get(&identifier).unwrap();

        assert_eq!(val, Literal::Boolean(true));

        env.define(key_val_2.0, key_val_2.1);

        let val = env.get(&identifier).unwrap();

        assert_eq!(val, Literal::Boolean(false));
    }

    #[test]
    fn test_returns_an_error_if_not_found() {
        let env = Environment::new();

        let identifier = Token::new(TokenType::Identifier, "no_var".to_string(), 0, None);

        let result = env.get(&identifier);
        assert!(result.is_err());
    }
}
