pub mod vm;
pub mod tokens;

use parser::Parser;
use self::tokens::Token;
use self::vm::Machine;


pub fn eval<'a>(code: &str) -> Token {
    let parser = Parser::parse(code);
    let mut environment = Machine::new();
    environment.eval_parser(&parser.tree)
}


#[cfg(test)]
mod test {
    use runtime::eval;
    use runtime::tokens::Token;

  #[test]
    fn test_define_with_add_string() {
        let runtime = eval(r#"
        (define r "Hello")
        (cons r " " "World")"#);
        match runtime {
            Token::None => assert!(false),
            Token::Str(result) => assert_eq!(result, "Hello World"),
            _ => {
                println!("{:?} Not a string", runtime);
                assert!(false, "NaN");
            }
        }
    }


  #[test]
    fn test_define_with_add_numbers() {
        let runtime = eval(r#"
        (define r 30)
        (+ r 40)"#);
        match runtime {
            Token::None => assert!(false),
            Token::Int(result) => assert_eq!(result, 70),
            _ => {
                println!("{:?} Nan", runtime);
                assert!(false, "NaN");
            }
        }
    }

  #[test]
    fn test_define() {
        let runtime = eval(r#"(define r 30)"#);
        match runtime {
            Token::None => assert!(true),
            Token::Error(_) => assert!(false),
            _ => {
                println!("{:?} Error", runtime);
                assert!(false, "No error");
            }
        }
    }



  #[test]
    fn test_define_too_many_parameters() {
        let runtime = eval(r#"(define r 30 40)"#);
        match runtime {
            Token::None => assert!(false),
            Token::Error(_result) => assert!(true),
            _ => {
                println!("{:?} Error", runtime);
                assert!(false, "No error");
            }
        }
    }

    #[test]
    fn test_cons() {
        let runtime = eval(r#"(cons "Hello " "du " "da")"#);
        match runtime {
            Token::None => assert!(false),
            Token::Str(result) => assert_eq!(*result, String::from("Hello du da")),
            _ => assert!(false, "It's not a string")
        }
    }

    #[test]
    fn test_add() {
        let runtime = eval(r#"(+ 10 20 30 40)"#);
        match runtime {
            Token::None => assert!(false),
            Token::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }

   #[test]
    fn test_add_nested() {
        let runtime = eval(r#"
        (+ 10 20
           (+ 30 40))"#);
        match runtime {
            Token::None => assert!(false),
            Token::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }


   #[test]
    fn test_add_nested_with_float() {
        let runtime = eval(r#"
        (+ 10 20.5
           (+ 30 40.5))"#);
        match runtime {
            Token::None => assert!(false),
            Token::Float(result) => assert_eq!(result, 101.0),
            _ => assert!(false, "NaN")
        }
    }

}