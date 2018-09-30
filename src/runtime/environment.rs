use std::collections::HashMap;
use std::boxed::Box;
use std::rc::Rc;


#[derive(Debug)]
pub enum ResultValue {
    None,
    Int(i64),
    Float(f64),
    Str(String),
    Func(),
    Error()
}

#[derive(Debug)]
pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    values: HashMap<String, ResultValue>
}

impl<'a> Environment<'a> {

    pub fn child(&'a self) -> Self {
        Environment{parent: Some(self),
                    values: HashMap::new()}
    }

    pub fn new() -> Self {
        Environment{parent: Option::None,
                    values: HashMap::new()}
    }

}