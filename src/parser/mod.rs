
use std::string::String;
use std::vec::Vec;
use std::num::ParseIntError;

struct Token {
    value: String,
    token: Vec<Vec<Token>>
}

pub struct Parser {
    tokens : Vec<Token>,
    tree : Token
}

impl Parser {

    pub fn new() -> Parser {
        Parser{tokens: Vec::new(), tree: Token{value: String::new(), token: Vec::new()}}
    }

    pub fn tokenize(&mut self, code: String) {
        let code_with_spaces = code.replace("(", "( ").replace(")", " )");
        for token in code_with_spaces.split(" "){
            self.tokens.push(Token{value: String::from(token), token: Vec::new()});
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseIntError> {
        self.tree = self.read_from_token();
        Ok(())
    }

    fn read_from_token(&mut self) -> Token {
        let mut result = Vec::new();
        while !self.tokens.is_empty() {
            let token = self.tokens.remove(0).value;
            match token.as_str() {
                "(" => {
                    let recur = self.read_from_token();
                    return Token{value: String::new(), token: result.push(recur)}
                },
                ")" => return Token{value: String::new(), token: result},
                _ => result.push(Token{value: token, token: Vec::new()})
            }
        };

        Token{value: String::new(), token: result}
    }
}

#[cfg(test)]
mod tests {
    use parser::Parser;

    #[test]
    fn test_tokenizer(){
        let mut parser = Parser::new();
        parser.tokenize(String::from("(begin (define r 10) (* pi (* r r)))"));
        assert_eq!(parser.tokens[0].value, "(");
        assert_eq!(parser.tokens[1].value, "begin");
        assert_eq!(parser.tokens[2].value, "(");
        assert_eq!(parser.tokens[3].value, "define");
        assert_eq!(parser.tokens[4].value, "r");
    }

    #[test]
    fn test_parser(){
        let mut parser = Parser::new();
        parser.tokenize(String::from("(begin (define r 10) (* pi (* r r)))"));
        assert!(parser.parse().is_ok());
        assert_eq!(parser.tree.token.unwrap()[0].value, "begin")
    }
}