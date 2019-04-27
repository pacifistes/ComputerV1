use crate::operand::Operand;
use crate::operator::Operator;
use colored::*;

macro_rules! clear_stone {
	($y: expr) => {
		 !(0b11 << ($y * 2) as u64)
	};
}

macro_rules! clear_stone {
	($y: expr) => {
		 !(0b11 << ($y * 2) as u64)
	};
}

pub fn negative_discriminant(a: f64, b: f64, c: f64, d: f64) {
    println!("{}", "Discriminanat is strictly negative, the two solutions are: ".white().bold());
    println!("({} + i * sqrt({})) / {})", -b, -d, a * 2.0);
    println!("({} - i * sqrt({})) / {})", -b, -d, a * 2.0);
}

pub fn null_discriminant(a: f64, b: f64) {
    assert_ne!(a, 0.00);
    println!("{}", "Discriminant is null, the solutions is :".white().bold());
    println!("{}", -b / (2.0 * a));
}

pub fn positive_discriminant(a: f64, b: f64, c: f64, d: f64) {
    assert_ne!(a, 0.00);
    println!("{}", "Discriminanat is strictly positive, the two solutions are: ".white().bold());
    println!("{}", (-b - d.sqrt()) / (2.0 * a));
    println!("{}", (-b + d.sqrt()) / (2.0 * a));
}

pub fn second_degree(a: f64, b: f64, c: f64) {
    let d: f64 = b * b - 4.0 * a * c;

    match d {
        negative if negative < 0.0 => negative_discriminant(a, b, c, d),
        positive if positive > 0.0 => positive_discriminant(a, b, c, d),
        _ => null_discriminant(a, b),
    };
}

pub fn first_degree(b: f64, c: f64) {
    assert_ne!(b, 0.00);
    println!("{}", "The solution is:".white().bold());
    println!("{}", -c / b);
}

pub fn zero_degree(c: f64) {
    if (c == 0.0) {
        println!("{}", "The solutions are all real numbers".white().bold());
    }
    else {
        println!("{}", "There is no solution".white().bold())
    }
}

pub fn equation_to_string(operands: &Vec<Operand>, operators: &Vec<Operator>) -> String {
    if operands.is_empty() {
        return String::from("0 = 0");
    }
    let mut equation: String =
        (0..(operands.len() * 2 - 1)).fold(String::new(), |mut equation, i| {
            if i % 2 == 0 {
                equation.push_str(operands[i / 2].to_string().as_str())
            } else {
                equation.push_str(operators[i / 2].as_str())
            }
            equation
        });
    equation.push_str(" = 0");
    equation
}

pub fn next_operator_position(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) -> Option<usize> {
    let position = operators.iter().position(|operator| *operator == Operator::Div || *operator == Operator::Mul);
    if position.is_none() {
        operators.iter().enumerate().position(|(index, operator)| {
            operands[index].x_power == operands[index + 1].x_power
        })
    }
    else {
        position
    }
}

pub fn clear_list(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) {
    loop {
        let position = operands.iter().position(|operand| {
            operand.value == 0.0
        });
        if position.is_none() {
            break;
        }
        else {
            let position = position.unwrap();
            let operand = operands.remove(position);
            if !operators.is_empty() {
                let operator = operators.remove(position);
                if (position == 0 && operator == Operator::Sub) {
                    operands[0].value = -operands[0].value;
                }
            }
        }
    }
}

pub fn reduce(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) {
    println!("{}", "Initial expression representation:".underline());
    println!("{}", equation_to_string(operands, operators));
    println!("{}", "Reduce steps:".underline());
    loop {
        let position = next_operator_position(operands, operators);
        if position.is_none() {
            break;
        }
        let position = position.unwrap();
        let a = operands.remove(position);
        let b = operands.remove(position);
        let operator = operators.remove(position);
        let new_operand = match operator {
            Operator::Add => a.add(b),
            Operator::Sub => a.sub(b),
            Operator::Mul => a.mul(b),
            Operator::Div => a.div(b),
        };
        operands.insert(position, new_operand);
        println!("{}", equation_to_string(operands, operators));
    }
    clear_list(operands, operators);
    println!("{}", "Clear value equal to zero:".underline());
    println!("{}", equation_to_string(operands, operators));
}

pub fn find_abc(operands: Vec<Operand>, operators: Vec<Operator>) -> (f64, f64, f64) {
    let get_value = |value: f64, index: usize, operators: &Vec<Operator>| -> f64 {
        if index != 0 && operators[index - 1] == Operator::Sub {
            -value
        }
        else {
            value
        }
    };
    operands.iter().enumerate().fold((0.0, 0.0, 0.0), |mut abc, (index, operand)| {
        match operand.x_power {
            0 => abc.2 = get_value(operand.value, index, &operators),
            1 => abc.1 = get_value(operand.value, index, &operators),
            _ => abc.0 = get_value(operand.value, index, &operators),
        };
        abc
    })
}

pub fn resolve(mut operands: Vec<Operand>, mut operators: Vec<Operator>) {
    reduce(&mut operands, &mut operators);
    let is_invalid_expression = operands.iter().any(|operand| {
        operand.x_power >= 3 || operand.x_power < 0
    });
    if is_invalid_expression {
        println!(
                "{}{}",
                "Error".red().bold(),
                ": The expression must be a polynomial expression lower or equal to 2"
                    .white()
                    .bold(),
            );
        return;
    }
    let (a, b, c) = find_abc(operands, operators);
    match (a, b, c) {
        (a, b, c) if a != 0.0 => second_degree(a, b, c),
        (a, b, c) if b != 0.0 => first_degree(b, c),
        _ => zero_degree(c),
    }
}
