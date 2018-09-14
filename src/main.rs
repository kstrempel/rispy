pub mod parser;

use std::string::String;

fn main() {
    println!("Hello, rispy!");
    let mut parser = parser::Parser::new();
    parser.parse(String::from("hello du da")).is_ok();
}
