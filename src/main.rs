mod parser;
mod operand;
mod operator;
mod resolver;
mod tests;

use crate::parser::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run --release \"polynomial expression lower or equal to two\"");
        return;
    }
    let tmp: i32 = match parse(&args[1]) {
        Ok(tmp) => tmp,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
}
