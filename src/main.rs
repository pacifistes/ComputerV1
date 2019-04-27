#[macro_use]
extern crate lazy_static;

mod operand;
mod operator;
mod parser;
mod resolver;
mod tests;

use crate::operand::Operand;
use crate::operator::Operator;
use crate::resolver::resolve;
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
    let (operands, operators): (Vec<Operand>, Vec<Operator>) = match parse(args[1].clone()) {
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
    resolve(operands, operators);
}
