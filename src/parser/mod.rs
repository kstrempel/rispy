
use std::string::String;
use std::vec::Vec;
use std::num::ParseIntError;

struct Token {
    value: String
}

pub struct Parser {
    tokens : Vec<Token>
}

impl Parser {

    pub fn new() -> Parser {
        Parser{tokens: Vec::new()}
    }

    pub fn parse<'a>(&mut self, code: String) -> Result<(), ParseIntError> {
        self.tokens.push(Token{value:code});
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    #[test]
    fn test_parser(){
        let mut parser = Parser::new();
        let result = parser.parse(String::from("(begin (define r 10) (* pi (* r r)))"));
        assert!(result.is_ok());
        assert_eq!(parser.tokens[0].value, "(");
    }
}