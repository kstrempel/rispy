use std::collections::HashMap;


#[derive(Debug)]
pub enum ResultValue {
    None,
    Int(i64),
    Str(String),
    Func(),
    Error()
}

#[derive(Debug)]
pub struct Environment {
    parent: Box<Option<Environment>>,
    values: HashMap<String, ResultValue>
}

impl Environment {

    fn new(parent: Option<Environment>) -> Environment {
        Environment{parent: Box::new(parent), values: HashMap::new()}
    }

    pub fn new_empty() -> Environment {
        Environment::new(Option::None)
    }

}