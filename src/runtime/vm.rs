use std::collections::HashMap;
use std::slice::Iter;

use runtime::tokens::Token;


#[derive(Debug)]
pub struct Machine {
    environment: Vec<HashMap<String, Token>>,
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

    pub fn eval_parser(&mut self, tree: &Vec<Token>) -> Token {
        let mut iter = tree.into_iter();
        let mut result = Token::None;
        self.push_environment();
        while let Some(token) = iter.next() {
            match token {
                Token::Atom(symbol) => {
                    result = self.match_atom(symbol, &mut iter);
                }
                Token::Block(subs) => {
                    result = self.eval_parser(subs);
                }
                _ => {
                    println!("Panic at {:?}", tree);
                    result = Token::None;
                }
            }
        }

        self.pop_environment();
        result
    }

    fn match_atom(&mut self, symbol: &String, iter: &mut Iter<Token>) -> Token {
        let str_symbol = symbol.as_str();
        match str_symbol {
            "cons" => {
                let mut result = String::new();
                while let Some(token) = iter.next() {
                    match token {
                        Token::Str(s) => result.push_str(s.as_str()),
                        Token::Atom(var_name) => {
                            println!("{:?}", self.environment);
                            let token = self.get_variable(var_name).unwrap();
                            match token {
                                Token::Str(s) => result.push_str(s.as_str()),
                                Token::Error(error) => println!("{}", error),
                                _ => println!("Panic"),
                            }
                        },
                        Token::Block(tokens) => {
                            let value = self.eval_parser(tokens);
                            match value {
                                Token::Str(s) => result.push_str(s.as_str()),
                                Token::Error(error) => println!("{}", error),
                                _ => println!("Panic"),
                            }
                        },
                        _ => result.push_str("Wrong parameters for 'cons'"),
                    };
                }
                Token::Str(result)
            }
            "+" => {
                let mut result = 0.0;
                let mut int_only = true;
                while let Some(token) = iter.next() {
                    match token {
                        Token::Int(atom) => result += *atom as f64,
                        Token::Float(atom) => {
                            int_only = false;
                            result += atom
                        }
                        Token::Atom(var_name) => {
                            let token = self.get_variable(var_name).unwrap();
                            match token {
                                Token::Int(atom) => result += *atom as f64,
                                Token::Float(atom) => {
                                    int_only = false;
                                    result += atom
                                }
                                Token::Error(error) => println!("{}", error),
                                _ => println!("Panic"),
                            }
                        }
                        Token::Block(tokens) => {
                            let value = self.eval_parser(tokens);
                            match value {
                                Token::Int(atom) => result += atom as f64,
                                Token::Float(atom) => {
                                    int_only = false;
                                    result += atom
                                }
                                _ => println!("Panic"),
                            }
                        }
                        _ => println!("Panic"),
                    };
                }
                if int_only {
                    Token::Int(result as i64)
                } else {
                    Token::Float(result)
                }
            }
            "define" => {
                let name = iter.next();
                let value = iter.next();
                if let Some(_) = iter.next() {
                    Token::Error(String::from("Panic to many parameters for define"))
                } else {
                    match Machine::get_name_and_result(name, value) {
                        Ok((name, value)) => {
                            match self.set_namespace_value(name, value) {
                                Ok(_) => Token::None,
                                Err(e) => Token::Error(e)
                            }
                        },
                        _ => Token::Error(String::from("error in define"))
                    }
                }
            }
            _ => Token::Error(String::from("unknown atom")),
        }
    }

    fn get_name_and_result(name: Option<&Token>, value: Option<&Token>) -> Result<(String, Token), String> {
        let token_name = name.expect("error in define - name");
        let token_value = value.expect("error in define - value");

        let name = token_name.atom_2_string().unwrap();

        Ok((name, token_value.clone()))
    }
}
