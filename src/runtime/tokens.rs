use std::vec::Vec;


#[derive(Debug)]
pub enum Token {
    None,
    Int(i64),
    Float(f64),
    Str(String),
    Atom(String),
    Func(),
    Block(Vec<Token>),
    Error(String),
}

impl Token {
    pub fn atom_2_string(&self) -> Result<String, Token> {
        match self {
            Token::Atom(s) => Ok(s.clone()),
            _ => Err(Token::Error(String::from("Must be a string")))
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::None => Token::None,
            Token::Float(f) => Token::Float(*f),
            Token::Int(i) => Token::Int(*i),
            Token::Str(s) => Token::Str(s.clone()),
            Token::Atom(s) => Token::Atom(s.clone()),
            Token::Func() => Token::Func(),
            Token::Block(vec) => Token::Block(vec.clone()),
            Token::Error(s) => Token::Error(s.clone())
        }
    }
}