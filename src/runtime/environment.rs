use std::collections::HashMap;

use runtime::tokens::Token;


#[derive(Debug)]
pub struct Machine {
    pub environment: Vec<HashMap<String, Token>>,
}


impl Machine {
    pub fn new() -> Self {
        Machine {
            environment: vec![HashMap::new()],
        }
    }

    pub fn pop_environment(&mut self) {
        self.environment.pop();
    }

    pub fn push_environment(&mut self) {
        self.environment.push(HashMap::new());
    }

    pub fn get_variable(&self, name: &String) -> Result<&Token, String> {
        if let Some(result_value) = self.environment[0].get(name) {
            return Ok(result_value);
        }

        Err(String::from("unknown variable name"))
    }

    pub fn set_namespace_value(&mut self, name: String, value: Token) -> Result<(), String> {
        if self.environment[0].contains_key(&name) {
            Err(String::from("Variable already exsists"))
        } else {
            self.environment[0].insert(name.clone(), value);
            Ok(())
        }
    }
}