
#[derive(Debug)]
pub enum Value {
    None,
    Int(i64),
    Float(f64),
    Str(String),
    Atom(String),
    Func(),
    Error(String),
}

