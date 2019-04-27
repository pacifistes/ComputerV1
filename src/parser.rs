use crate::operator::Operator;
use crate::operand::Operand;
use crate::resolver::equation_to_string;
use regex::Regex;
use regex::Captures;

pub fn get_value(s: &String) -> Result<f64, &'static str> {
    match s.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Invalid String. Number representing no power number must be valid double."),
    }
}

pub fn get_x_power(s1: &String, s2: &String) -> isize {
    let x_power = |s: &String| -> isize {
        match s.parse::<isize>() {
            Ok(value) => value,
            Err(_) => 1
        }
    };
    let power = x_power(s1);
    if (power == 1) {
        x_power(s2)
    }
    else {
        power
    }
}

pub fn get_operand(expression: &str, equal_passed: bool, index: &mut usize) -> Result<Operand, &'static str> {
    lazy_static! {
        static ref regex: Regex = {
            let float = r"(\-?[0-9]+(?:\.[0-9]+)?)";
            let puissance = r"X(:?\^([0-9]+))?";
            let operand_expression = format!(r"^(?:{}(?:\*{})?|{})", float, puissance, puissance);
            Regex::new(&operand_expression[..]).unwrap()
        };
    }
    let captures: Vec<String> = match regex.captures(expression) {
        Some(captures) => captures.iter().map(|capture| {
            if capture.is_some() {
                String::from(capture.unwrap().as_str())
            }
            else {
                String::new()
            }
        }).collect(),
        None => return Err("Invalid String. The string must be a polynomial expression."),
    };
    *index += captures[0].len();
    let mut value = {
        if !captures[1].is_empty() {
            get_value(&captures[1])?
        }
        else {
            1.0
        }
    };
    if (equal_passed) {
        value = -value;
    }
    let x_power = get_x_power(&captures[3], &captures[5]);
    Ok(Operand::new(value, x_power))
}

pub fn get_operator(expression: &str) -> Result<Operator, &'static str> {
    lazy_static! {
        static ref regex: Regex = {
            let operator_expression = r"^([\+\-\*/])";
            Regex::new(operator_expression).unwrap()
        };
    }
    let captures = match regex.captures(expression) {
        Some(captures) => captures,
        None => return Err("Invalid String. The string must be a polynomial expression."),
    };
    Ok(Operator::new(&captures[0]))
}

pub fn parse(mut expression: String) -> Result<(Vec<Operand>, Vec<Operator>), &'static str> {
    let mut equal_passed: bool = false;
    let mut waiting_operand: bool = true;
    let mut operands: Vec<Operand> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    let mut index = 0;
    expression.retain(|c| !c.is_whitespace());
    let len = expression.len();

    while index < len {
        if !waiting_operand && !equal_passed && expression.as_bytes()[index] == '=' as u8 {
            index += 1;
            waiting_operand = true;
            equal_passed = true;
            operators.push(Operator::Add);
        }
        if (waiting_operand) {
            let operand = get_operand(&expression[index..], equal_passed, &mut index)?;
            operands.push(operand);
            waiting_operand = !waiting_operand;
        }
        else {
            let operator = get_operator(&expression[index..])?;
            operators.push(operator);
            waiting_operand = !waiting_operand;
            index += 1;
        }
    }
    if waiting_operand {
        Err("Invalid String. The string must be a polynomial expression.")
    }
    else {
        Ok((operands, operators))
    }
}
