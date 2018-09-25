extern crate regex;

mod runtime;
mod parser;

use runtime::Runtime;

fn main() {
    let runtime = Runtime::eval(r#"(cons "Hello " "du " "da ")"#);
    println!("{:?}", runtime);
}
