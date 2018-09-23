use std::vec::IntoIter;
use std::ops::Add;
use parser::{Parser, Token};


#[derive(Debug)]
pub enum Runtime {
    None,
    Int(i32),
    Str(String),
    Func()
}

impl Runtime {

    pub fn eval(code: &str) -> Runtime {
        let runtime = Runtime::None;
        let parser = Parser::parse(code);
        runtime.eval_parser(parser.tree)
    }

    fn eval_parser(self, tree: Vec<Token>) -> Runtime {
        let mut iter = tree.into_iter();
        while let Some(token) = iter.next() {
            match token {
                Token::Atom(symbol) => return self.match_atom(symbol, iter),
                Token::Subs(subs) => return self.eval_parser(subs)
            }
        }

        Runtime::None
    }

    fn match_atom(self, symbol: String, mut iter: IntoIter<Token>) -> Runtime {
        let str_symbol = symbol.as_str();
        match str_symbol {
            "cons" => {
                let mut result = String::new();
                while let Some(token) = iter.next() {
                    match token {
                        Token::Atom(symbol) => result.push_str(symbol.as_str()),
                        _ => result.push_str("no subs")
                    };
                };
                return Runtime::Str(result)
            },
            _ => println!("unknown atom")
        }

        Runtime::None
    }
}

#[cfg(test)]
mod test {
    use runtime::Runtime;

    #[test]
    fn test_addition() {
        let runtime = Runtime::eval("(cons \"Hello \" \"du\" \"da\")");
        match runtime {
            Runtime::None => assert!(true),
            Runtime::Str(result) => assert_eq!(result, String::from("Hello du da")),
            _ => assert!(false, "It's not none")
        }
    }
}