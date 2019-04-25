mod operand;
mod operator;
mod parser;
mod resolver;
mod tests;

use crate::parser::*;
use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "{}{}",
            "Usage".yellow().bold(),
            ": cargo run --release \"polynomial expression lower or equal to two\""
                .white()
                .bold()
        );
        return;
    }
    let tmp: i32 = match parse(&args[1]) {
        Ok(tmp) => tmp,
        Err(error) => {
            println!(
                "{}{}{}",
                "Error".red().bold(),
                ": ".white().bold(),
                error.white().bold()
            );
            return;
        }
    };
}
