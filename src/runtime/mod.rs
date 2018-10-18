pub mod vm;

use parser::Parser;

use self::vm::{Machine, Value};

pub fn eval<'a>(code: &str) -> Value {
    let parser = Parser::parse(code);
    let mut environment = Machine::new();
    environment.eval_parser(&parser.tree)
}


#[cfg(test)]
mod test {
    use runtime::eval;
    use runtime::Value;

  #[test]
    fn test_define_with_add() {
        let runtime = eval(r#"
        (define r 30)
        (+ r 40)"#);
        match runtime {
            Value::None => assert!(false),
            Value::Int(result) => assert_eq!(result, 70),
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
            Value::None => assert!(true),
            Value::Error(_) => assert!(false),
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
            Value::None => assert!(false),
            Value::Error(_result) => assert!(true),
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
            Value::None => assert!(false),
            Value::Str(result) => assert_eq!(*result, String::from("Hello du da")),
            _ => assert!(false, "It's not a string")
        }
    }

    #[test]
    fn test_add() {
        let runtime = eval(r#"(+ 10 20 30 40)"#);
        match runtime {
            Value::None => assert!(false),
            Value::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }

   #[test]
    fn test_add_nested() {
        let runtime = eval(r#"
        (+ 10 20
           (+ 30 40))"#);
        match runtime {
            Value::None => assert!(false),
            Value::Int(result) => assert_eq!(result, 100),
            _ => assert!(false, "NaN")
        }
    }


   #[test]
    fn test_add_nested_with_float() {
        let runtime = eval(r#"
        (+ 10 20.5
           (+ 30 40.5))"#);
        match runtime {
            Value::None => assert!(false),
            Value::Float(result) => assert_eq!(result, 101.0),
            _ => assert!(false, "NaN")
        }
    }

}