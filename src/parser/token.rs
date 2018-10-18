
use std::vec::Vec;

use runtime::value::Value;


#[derive(Debug)]
pub enum Token {
    Atom(String),
    AtomString(String),
    AtomInt(i64),
    AtomFloat(f64),
    Subs(Vec<Token>)
}


impl Token {
    pub fn token_2_value(&self) -> Result<Value, Value> {
        match self {
            Token::Atom(s) => Ok(Value::Atom(s.clone())),
            Token::AtomFloat(f) => Ok(Value::Float(f.clone())),
            Token::AtomInt(i) => Ok(Value::Int(i.clone())),
            Token::AtomString(s) => Ok(Value::Str(s.clone())),
            _ => Err(Value::Error(String::from("Unknown transformation")))
        }
    }

    pub fn atom_2_string(&self) -> Result<String, Value> {
        match self {
            Token::Atom(s) => Ok(s.clone()),
            _ => Err(Value::Error(String::from("Must be a string")))
        }
    }
}