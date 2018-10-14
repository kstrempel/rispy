use std::collections::HashMap;
use std::slice::Iter;

use parser::Token;

#[derive(Debug)]
pub enum ResultValue {
    None,
    Int(i64),
    Float(f64),
    Str(String),
    Func(),
    Error(String),
}

impl ResultValue {

}


#[derive(Debug)]
pub struct Machine {
    environment: Vec<HashMap<String, ResultValue>>,
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

    pub fn get_variable(&self, name: &String) -> Result<&ResultValue, String> {
        if let Some(result_value) = self.environment[0].get(name) {
            return Ok(result_value);
        }

        Err(String::from("unknown variable name"))
    }

    pub fn set_namespace_value(
        &mut self,
        name: String,
        value: ResultValue,
    ) -> Result<(), String> {
        if self.environment[0].contains_key(&name) {
            Err(String::from("Variable already exsists"))
        } else {
            self.environment[0].insert(name.clone(), value);
            Ok(())
        }
    }

    pub fn eval_parser(&mut self, tree: &Vec<Token>) -> ResultValue {
        let mut iter = tree.into_iter();
        let mut result = ResultValue::None;
        self.push_environment();
        while let Some(token) = iter.next() {
            match token {
                Token::Atom(symbol) => {
                    result = self.match_atom(symbol, &mut iter);
                }
                Token::Subs(subs) => {
                    result = self.eval_parser(subs);
                }
                _ => {
                    println!("Panic at {:?}", tree);
                    result = ResultValue::None;
                }
            }
        }

        self.pop_environment();
        result
    }

    fn match_atom(&mut self, symbol: &String, iter: &mut Iter<Token>) -> ResultValue {
        let str_symbol = symbol.as_str();
        match str_symbol {
            "cons" => {
                let mut result = String::new();
                while let Some(token) = iter.next() {
                    match token {
                        Token::AtomString(atom) => result.push_str(atom.as_str()),
                        _ => result.push_str("Wrong parameters for 'cons'"),
                    };
                }
                ResultValue::Str(result)
            }
            "+" => {
                let mut result = 0.0;
                let mut int_only = true;
                while let Some(token) = iter.next() {
                    match token {
                        Token::AtomInt(atom) => result += *atom as f64,
                        Token::AtomFloat(atom) => {
                            int_only = false;
                            result += atom
                        }
                        Token::Atom(var_name) => {
                            println!("{:?}", var_name);
                            let value = self.get_variable(var_name).unwrap();
                            match value {
                                ResultValue::Int(atom) => result += *atom as f64,
                                ResultValue::Float(atom) => {
                                    int_only = false;
                                    result += atom
                                }
                                ResultValue::Error(error) => println!("{}", error),
                                _ => println!("Panic"),
                            }
                        }
                        Token::Subs(tokens) => {
                            let result_value = self.eval_parser(tokens);
                            match result_value {
                                ResultValue::Int(atom) => result += atom as f64,
                                ResultValue::Float(atom) => {
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
                    ResultValue::Int(result as i64)
                } else {
                    ResultValue::Float(result)
                }
            }
            "define" => {
                let name = iter.next();
                let value = iter.next();
                if let Some(_) = iter.next() {
                    ResultValue::Error(String::from("Panic to many parameters for define"))
                } else {
                    match Machine::get_name_and_result(name, value) {
                        Ok((name, value)) => {
                            match self.set_namespace_value(name, value) {
                                Ok(_) => ResultValue::None,
                                Err(e) => ResultValue::Error(e)
                            }
                        },
                        _ => ResultValue::Error(String::from("error in define"))
                    }
                }
            }
            _ => ResultValue::Error(String::from("unknown atom")),
        }
    }

    fn get_name_and_result(
        name: Option<&Token>,
        value: Option<&Token>,
    ) -> Result<(String, ResultValue), ResultValue> {
        let token_name = name.expect("error in define - name");
        let token_value = value.expect("error in define - value");

        let name = match token_name {
            Token::Atom(name) => name.clone(),
            _ => String::from("_")
        };

        let value = match token_value {
            Token::AtomInt(value) => ResultValue::Int(*value),
            _ => ResultValue::Error(String::from("Error in define - value type"))
        };

        Ok((name, value))
    }
}
