use std::vec::IntoIter;
use parser::{Parser, Token};


#[derive(Debug)]
pub enum Runtime {
    None,
    Int(i64),
    Str(String),
    Func(),
    Error()
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
                Token::Subs(subs) => return self.eval_parser(subs),
                _ => return Runtime::None
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
                        Token::AtomString(atom) => result.push_str(atom.as_str()),
                        _ => result.push_str("Wrong parameters for 'cons'")
                    };
                };
                Runtime::Str(result)
            },
            "+" => {
                let mut result = 0;
                while let Some(token) = iter.next() {
                    match token {
                        Token::AtomInt(atom) => result += atom,
                        _ => println!("Panic")
                    };
                };
                Runtime::Int(result)
            },
            _ => {
                println!("unknown atom");
                Runtime::Error()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use runtime::Runtime;

    #[test]
    fn test_cons() {
        let runtime = Runtime::eval(r#"(cons "Hello " "du " "da")"#);
        match runtime {
            Runtime::None => assert!(false),
            Runtime::Str(result) => assert_eq!(result, String::from("Hello du da")),
            _ => assert!(false, "It's not a string")
        }
    }

    #[test]
    fn test_add() {
        let runtime = Runtime::eval(r#"(+ 10 20 30 40)"#);
        match runtime {
            Runtime::None => assert!(false),
            Runtime::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }

}