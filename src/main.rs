extern crate regex;

mod runtime;
mod parser;

use runtime::Runtime;

fn main() {
    let runtime = Runtime::eval("(cons 10 10)");
    println!("{:?}", runtime);
}
