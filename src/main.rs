extern crate regex;

mod runtime;
mod parser;

use runtime::eval;

fn main() {
    let runtime = eval(r#"(cons "Hello " "du " "da ")"#);
    println!("{:?}", runtime);
}
