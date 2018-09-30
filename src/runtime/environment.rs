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
    Error(String)
}

#[derive(Debug)]
pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    values: HashMap<String, ResultValue>
}

impl<'a> Environment<'a> {

    pub fn child(&'a self) -> Environment {
        Environment{parent: Some(self),
                    values: HashMap::new()}
    }

    pub fn new() -> Self {
        Environment{parent: Option::None,
                    values: HashMap::new()}
    }

    pub fn get_variable(&'a self, name: &String) -> Result<&ResultValue, String> {
        if let Some(result_value) = self.values.get(name) {
            return Ok(result_value);
        }

        Err(String::from("unknown variable name"))
    }

    pub fn eval_parser(&mut self, tree: &'a Vec<Token>) -> ResultValue {
        let mut iter = tree.into_iter();
        let mut result = ResultValue::None;
        while let Some(token) = iter.next() {
            match token {
                Token::Atom(symbol) => {
                    result = self.match_atom(symbol, &mut iter);
                },
                Token::Subs(subs) => {
                    let mut child = self.child();
                    result = child.eval_parser(subs);
                }
                _ => {
                    println!("Panic at {:?}", tree);
                    result = ResultValue::None;
                }
            }
        }

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
                        _ => result.push_str("Wrong parameters for 'cons'")
                    };
                };
                ResultValue::Str(result)
            },
            "+" => {
                let mut result = 0.0;
                let mut int_only = true;
                while let Some(token) = iter.next() {
                    match token {
                        Token::AtomInt(atom) => result += *atom as f64,
                        Token::AtomFloat(atom) => {
                                    int_only = false;
                                    result += atom
                        },
                        Token::Atom(var_name) => {
                            let value = self.get_variable(var_name).unwrap();
                            match value {
                                ResultValue::Int(atom) => result += *atom as f64,
                                ResultValue::Float(atom) => {
                                    int_only = false;
                                    result += atom
                                },
                                ResultValue::Error(error) => println!("{}",error),
                                _ => println!("Panic")
                            }
                        },
                        Token::Subs(tokens) => {
                            let mut child_environment = self.child();
                            let result_value = child_environment.eval_parser(tokens);
                            match result_value {
                                ResultValue::Int(atom) => result += atom as f64,
                                ResultValue::Float(atom) => {
                                    int_only = false;
                                    result += atom
                                },
                                _ => println!("Panic")
                            }
                        },
                        _ => println!("Panic")
                    };
                };
                if int_only {
                    ResultValue::Int(result as i64)
                } else {
                    ResultValue::Float(result)
                }
            },
            "define" => {
                let _name = iter.next();
                let _value = iter.next();
                if let Some(_) = iter.next() {
                    ResultValue::Error(String::from("Panic to many parameters for define"))
                } else {
                    ResultValue::Int(10)
                }
            }
            _ => {
                ResultValue::Error(String::from("unknown atom"))
            }
        }
    }
}