extern crate regex;

use std::string::String;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Atom(String),
    AtomString(String),
    Subs(Vec<Token>)
}

#[derive(Debug)]
pub struct Parser {
    tokens : Vec<String>,
    pub tree : Vec<Token>
}

impl Parser {

    pub fn parse(code : &str) -> Parser {
        let mut result = Parser{tokens: Vec::new(), tree: Vec::new()};
        result.tokenize(code);
        result.tree = result.read_from_token();
        result
    }

    fn tokenize(&mut self, code: &str) {
        let code_with_spaces = code.replace("(", "( ").replace(")", " )");
        for token in code_with_spaces.split(" "){
            self.tokens.push(String::from(token));
        }
    }


    fn analyse_atom(&self, token_block: &str) -> Token {
        let regex = Regex::new(r#""(?P<token>.*)""#).unwrap();
        let cap = regex.captures(token_block);
        match cap {
            Some(content) => Token::AtomString(String::from(content.name("token").unwrap().as_str())),
            _ => Token::Atom(String::from(token_block))
        }
    }

    fn read_from_token(&mut self) -> Vec<Token> {
        let mut token : Vec<Token> = Vec::new();

        while !self.tokens.is_empty() {
            let token_block = self.tokens.remove(0);
            match token_block.as_str() {
                "(" => token.push(Token::Subs(self.read_from_token())),
                ")" => return token,
                _ => {
                    let atom = self.analyse_atom(&token_block);
                    token.push(atom)
                }
            }
        };

        token
    }
}

#[cfg(test)]
mod tests {
    use parser::{Parser, Token};

    fn check_define(subs: &Vec<Token>){
        match subs[0] {
            Token::Atom(ref c) => assert_eq!("define", c),
            _ => assert!(false, "define")
        }

        match subs[1] {
            Token::Atom(ref c) => assert_eq!("r", c),
            _ => assert!(false, "r")
        }

        match subs[2] {
            Token::Atom(ref c) => assert_eq!("10", c),
            _ => assert!(false, "10")
        }
    }

    fn check_pi(subs: &Vec<Token>) {
        match subs[0] {
            Token::Atom(ref c) => assert_eq!("*", c),
            _ => assert!(false, "*")
        }

        match subs[1] {
            Token::Atom(ref c) => assert_eq!("pi", c),
            _ => assert!(false, "pi")
        }
    }

    fn check_begin(subs: &Vec<Token>){
        println!("check begin {:?}", subs);

        match subs[0] {
            Token::Atom(ref c) => assert_eq!("begin", c),
            _ => assert!(false, "No begin")
        }

        match subs[1] {
            Token::Subs(ref c) => check_define(c),
            _ => assert!(false, "Check define")
        }

        match subs[2] {
            Token::Subs(ref c) => check_pi(c),
            _ => assert!(false, "Check pi")
        }
    }

    #[test]
    fn test_parser(){
        let parser = Parser::parse("(begin (define r 10) (* pi (* r r)))");
        match parser.tree[0] {
            Token::Subs(ref c) => check_begin(c),
            _ => assert!(false)
        }
    }
}