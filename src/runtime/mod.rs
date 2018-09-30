mod environment;

use std::slice::Iter;

use parser::{Parser, Token};

use self::environment::{Environment, ResultValue};


pub fn eval(code: &str) -> ResultValue {
    let parser = Parser::parse(code);
    let environment = Environment::new();
    eval_parser(&parser.tree, &environment)
}

fn eval_parser(tree: &Vec<Token>, environment: &Environment) -> ResultValue {
    let mut iter = tree.into_iter();
    let mut result = ResultValue::None;
    while let Some(token) = iter.next() {
        match token {
            Token::Atom(symbol) => {
                result = match_atom(symbol, &mut iter, environment);
            },
            Token::Subs(subs) => {
                result = eval_parser(subs, &environment.child());
            }
            _ => {
                println!("Panic at {:?}", tree);
                result = ResultValue::None;
            }
        }
    }

    result
}

fn match_atom(symbol: &String, iter: &mut Iter<Token>, environment: &Environment) -> ResultValue {
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
            let mut result = 0.0;
            let mut int_only = true;
            while let Some(token) = iter.next() {
                match token {
                    Token::AtomInt(atom) => result += *atom as f64,
                    Token::AtomFloat(atom) => {
                                int_only = false;
                                result += atom
                    },
                    Token::Subs(tokens) => {
                        let result_value = eval_parser(tokens, &environment.child());
                        match result_value {
                            ResultValue::Int(atom) => result += atom as f64,
                            ResultValue::Float(atom) => {
                                int_only = false;
                                result += atom
                            },
                            _ => println!("Panic")
                        }
                    },
                    _ => println!("Panic")
                };
            };
            if int_only {
                ResultValue::Int(result as i64)
            } else {
                ResultValue::Float(result)
            }
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
    fn test_define_with_add() {
        let runtime = eval(r#"
        (+ 10 10)
        (+ 30 40)"#);
        match runtime {
            ResultValue::None => assert!(false),
            ResultValue::Int(result) => assert_eq!(result, 70),
            _ => {
                println!("{:?} Nan", runtime);
                assert!(false, "NaN");
            }
        }
    }

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


   #[test]
    fn test_add_nested_with_float() {
        let runtime = eval(r#"
        (+ 10 20.5
           (+ 30 40.5))"#);
        match runtime {
            ResultValue::None => assert!(false),
            ResultValue::Float(result) => assert_eq!(result, 101.0),
            _ => assert!(false, "NaN")
        }
    }

}