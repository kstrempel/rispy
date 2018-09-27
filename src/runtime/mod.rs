mod environment;

use std::vec::IntoIter;

use parser::{Parser, Token};

use self::environment::{Environment, ResultValue};


pub fn eval(code: &str) -> ResultValue {
    let parser = Parser::parse(code);
    let mut environment = Environment::new_empty();
    eval_parser(environment, parser.tree)
}

fn eval_parser(mut environment: Environment, tree: Vec<Token>) -> ResultValue {
    let mut iter = tree.into_iter();
    while let Some(token) = iter.next() {
        match token {
            Token::Atom(symbol) => return match_atom(symbol, iter),
            Token::Subs(subs) => return eval_parser(environment, subs),
            _ => return ResultValue::None
        }
    }

    ResultValue::None
}

fn match_atom(symbol: String, mut iter: IntoIter<Token>) -> ResultValue {
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
            ResultValue::Str(result)
        },
        "+" => {
            let mut result = 0;
            while let Some(token) = iter.next() {
                match token {
                    Token::AtomInt(atom) => result += atom,
                    Token::Subs(tokens) => {
                        let mut environment = Environment::new_empty();
                        let result_value = eval_parser(environment, tokens);
                        match result_value {
                            ResultValue::Int(atom) => result += atom,
                            _ => println!("Panic")
                        }
                    },
                    _ => println!("Panic")
                };
            };
            ResultValue::Int(result)
        },
        _ => {
            println!("unknown atom");
            ResultValue::Error()
        }
    }
}


#[cfg(test)]
mod test {
    use runtime::eval;
    use runtime::ResultValue;

    #[test]
    fn test_cons() {
        let runtime = eval(r#"(cons "Hello " "du " "da")"#);
        match runtime {
            ResultValue::None => assert!(false),
            ResultValue::Str(result) => assert_eq!(result, String::from("Hello du da")),
            _ => assert!(false, "It's not a string")
        }
    }

    #[test]
    fn test_add() {
        let runtime = eval(r#"(+ 10 20 30 40)"#);
        match runtime {
            ResultValue::None => assert!(false),
            ResultValue::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }

   #[test]
    fn test_add_nested() {
        let runtime = eval(r#"
        (+ 10 20
           (+ 30 40))"#);
        match runtime {
            ResultValue::None => assert!(false),
            ResultValue::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }


}