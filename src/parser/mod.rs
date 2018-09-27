extern crate regex;

use std::string::String;
use std::vec::Vec;

use regex::RegexSet;

#[derive(Debug)]
pub enum Token {
    Atom(String),
    AtomString(String),
    AtomInt(i64),
    AtomFloat(f64),
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
        let mut token = String::new();
        let mut inside_string = false;
        for c in code.chars() {
            match c {
                '(' => {
                    if !token.is_empty() {
                        self.tokens.push(token.clone());
                    }
                    self.tokens.push(String::from("("));
                },
                ')' => {
                    if !token.is_empty() {
                        self.tokens.push(token.clone());
                        token.clear();
                    }
                    self.tokens.push(String::from(")"));
                },
                '"' => {
                    token.push(c);
                    if inside_string {
                        self.tokens.push(token.clone());
                        token.clear();
                        inside_string = false;
                    } else {
                        inside_string = true;
                    }
                },
                ' '|'\n' => {
                    if inside_string {
                        token.push(c);
                    } else {
                        if !token.is_empty(){
                            self.tokens.push(token.clone());
                        }
                        token.clear();
                    }
                },
                _ => token.push(c)
            }
        }
    }

    fn analyse_atom(&self, token_block: &str) -> Token {
        let set = RegexSet::new(&[
            r#"".*""#,              // 0 string
            r"\d+\.\d+",            // 2 float
            r"\d+",                 // 1 decimal
            r".+"]).unwrap();       // 3 the rest

        let matches : Vec<_> = set.matches(token_block).into_iter().collect();
        match matches[0] {
            0 => {
                let token = String::from(token_block);
                let len = token.len() - 1;
                Token::AtomString(String::from(&token[1..len]))
            },
            1 => {
                let num : f64 = token_block.parse().unwrap();
                Token::AtomFloat(num)
            },
            2 => {
                let num : i64 = token_block.parse().unwrap();
                Token::AtomInt(num)
            },
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
            Token::AtomInt(c) => assert_eq!(10, c),
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