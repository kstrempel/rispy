use std::slice::Iter;

use runtime::tokens::Token;
use runtime::environment::Machine;


impl Machine {
    pub fn match_atom(&mut self, symbol: &String, iter: &mut Iter<Token>) -> Token {
        let str_symbol = symbol.as_str();
        match str_symbol {
            "cons" => self.cons(iter),
            "+" => self.plus(iter),
            "define" => self.define(iter),
            _ => Token::Error(String::from("unknown atom")),
        }
    }

    fn cons(&mut self, iter: &mut Iter<Token>) -> Token {
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

    fn plus(&mut self, iter: &mut Iter<Token>) -> Token {
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

    fn define(&mut self, iter: &mut Iter<Token>) -> Token {
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
}