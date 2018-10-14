extern crate regex;

mod parser;
mod runtime;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use runtime::eval;
use runtime::vm::ResultValue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let runtime: Result<ResultValue, Error> = File::open(&args[1])
            .and_then(|mut f| {
                let mut content = String::new();
                f.read_to_string(&mut content)
                    .and_then(|_| Ok(eval(&content)))
                    .map_err(|e| e)
            }).map_err(|e| e);
        println!("{:?}", runtime.unwrap());
    } else {
        println!("No filename found");
    }
}
