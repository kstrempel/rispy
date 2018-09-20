use std::string::String;
use std::vec::Vec;
use std::num::ParseIntError;

#[derive(Debug)]
enum Token {
    Atom(String),
    Subs(Vec<Token>)
}

#[derive(Debug)]
pub struct Parser {
    tokens : Vec<String>,
    tree : Vec<Token>
}

impl Parser {

    pub fn new() -> Parser {
        Parser{tokens: Vec::new(), tree: Vec::new()}
    }

    pub fn tokenize(&mut self, code: String) {
        let code_with_spaces = code.replace("(", "( ").replace(")", " )");
        for token in code_with_spaces.split(" "){
            self.tokens.push(String::from(token));
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseIntError> {
        self.tree = self.read_from_token();
        Ok(())
    }

    fn read_from_token(&mut self) -> Vec<Token> {
        let mut token : Vec<Token> = Vec::new();

        while !self.tokens.is_empty() {
            let token_block = self.tokens.remove(0);
            match token_block.as_str() {
                "(" => token.push(Token::Subs(self.read_from_token())),
                ")" => return token,
                _ => token.push(Token::Atom(token_block))
            }
        };

        token
    }
}

#[cfg(test)]
mod tests {
    use parser::{Parser, Token};

    #[test]
    fn test_tokenizer(){
        let mut parser = Parser::new();
        parser.tokenize(String::from("(begin (define r 10) (* pi (* r r)))"));
        assert_eq!(parser.tokens[0], "(");
        assert_eq!(parser.tokens[1], "begin");
        assert_eq!(parser.tokens[2], "(");
        assert_eq!(parser.tokens[3], "define");
        assert_eq!(parser.tokens[4], "r");
    }

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
        let mut parser = Parser::new();
        parser.tokenize(String::from("(begin (define r 10) (* pi (* r r)))"));
        assert!(parser.parse().is_ok());
        match parser.tree[0] {
            Token::Subs(ref c) => check_begin(c),
            _ => assert!(false)
        }
    }
}