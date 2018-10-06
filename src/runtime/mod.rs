pub mod vm;

use parser::Parser;

use self::vm::{Machine, ResultValue};

pub fn eval<'a>(code: &str) -> ResultValue {
    let parser = Parser::parse(code);
    let mut environment = Machine::new();
    environment.eval_parser(&parser.tree)
}


#[cfg(test)]
mod test {
    use runtime::eval;
    use runtime::ResultValue;

  #[test]
    fn test_define_with_add() {
        let runtime = eval(r#"
        (define r 30)
        (+ r 40)"#);
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
    fn test_define() {
        let runtime = eval(r#"(define r 30)"#);
        match runtime {
            ResultValue::None => assert!(true),
            ResultValue::Error(_) => assert!(false),
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
            ResultValue::None => assert!(false),
            ResultValue::Error(_result) => assert!(true),
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
            ResultValue::None => assert!(false),
            ResultValue::Str(result) => assert_eq!(*result, String::from("Hello du da")),
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