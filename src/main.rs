mod runtime;
mod parser;

use runtime::Runtime;

fn main() {
    let runtime = Runtime::eval("(+ 10 10)");
    println!("{:?}", runtime);
}
