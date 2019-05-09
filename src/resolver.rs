use crate::operand::Operand;
use crate::operator::Operator;
use colored::*;

pub fn negative_discriminant(a: f64, b: f64, d: f64) {
    println!("{}", "Discriminant is strictly negative, the two solutions are: ".white().bold());
    println!("({} + i * sqrt({})) / {})", -b, -d, a * 2.0);
    println!("({} - i * sqrt({})) / {})", -b, -d, a * 2.0);
}

pub fn null_discriminant(a: f64, b: f64) {
    assert_ne!(a, 0.00);
    println!("{}", "Discriminant is null, the solutions is :".white().bold());
    println!("{}", -b / (2.0 * a));
}

pub fn positive_discriminant(a: f64, b: f64, d: f64) {
    assert_ne!(a, 0.00);
    println!("{}", "Discriminant is strictly positive, the two solutions are: ".white().bold());
    println!("{}", (-b - d.sqrt()) / (2.0 * a));
    println!("{}", (-b + d.sqrt()) / (2.0 * a));
}

pub fn second_degree(a: f64, b: f64, c: f64) {
    let d: f64 = b * b - 4.0 * a * c;
    println!("discriminant = {}^2 - 4 * {} * {}", b, a, c);
    println!("discriminant = {}", d);
    match d {
        negative if negative < 0.0 => negative_discriminant(a, b, d),
        positive if positive > 0.0 => positive_discriminant(a, b, d),
        _ => null_discriminant(a, b),
    };
}

pub fn first_degree(b: f64, c: f64) {
    assert_ne!(b, 0.00);
    println!("{}", "The solution is:".white().bold());
    println!("{}", -c / b);
}

pub fn zero_degree(c: f64) {
    if c == 0.0 {
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

pub fn next_operator_position(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) -> Option<(usize, usize)> {
    let position = operators.iter().position(|operator| *operator == Operator::Div || *operator == Operator::Mul);
    if position.is_none() {
        let pos1 = operands.iter().enumerate().position(|(index, operand_a)| {
            operands.iter().skip(index + 1).any(|operand_b| {
                operand_a.x_power == operand_b.x_power
            })
        });
        if pos1.is_some() {
            let pos1 = pos1.unwrap();
            let pos2 = operands.iter().skip(pos1 + 1).position(|operand_b| {
                operands[pos1].x_power == operand_b.x_power
            }).unwrap() + pos1 + 1;
            Some((pos1, pos2))
        }
        else {
            None
        }
    }
    else {
        let position = position.unwrap();
        Some((position, position + 1))
    }
}

pub fn sort_list(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) {
    let tmp_operands = operands.clone();
    operands.sort_by(|a, b| a.x_power.cmp(&b.x_power));
    let nbr_operands = operands.len();
    let mut sorted_operators: Vec<Operator> = operands
        .iter()
        .map(|operand| {
            let old_pos = tmp_operands.iter().position(|old_operand| {
                operand.x_power == old_operand.x_power
            }).unwrap();
            if old_pos == 0 {
                Operator::Add
            }
            else {
                operators[old_pos - 1]
            }
        })
        .collect();
    if sorted_operators.len() == nbr_operands {
        let operator = sorted_operators.remove(0);
        if operator == Operator::Sub {
            operands[0].value = -operands[0].value;
        }
    }
    *operators = sorted_operators;
}

pub fn reduce(operands: &mut Vec<Operand>, operators: &mut Vec<Operator>) {
    println!("{}", "Initial expression representation:".underline());
    println!("{}", equation_to_string(operands, operators));
    println!("{}", "Reduce steps:".underline());
    loop {
        let positions: Option<(usize, usize)> = next_operator_position(operands, operators);
        if positions.is_none() {
            break;
        }
        let (pos1, pos2) = positions.unwrap();
        let b = operands.remove(pos2);
        let a = operands.remove(pos1);
        let operator = operators.remove(pos2 - 1);
        let new_operand = match operator {
            Operator::Add => a.add(b),
            Operator::Sub => a.sub(b),
            Operator::Mul => a.mul(b),
            Operator::Div => a.div(b),
        };
        operands.insert(pos1, new_operand);
        println!("{}", equation_to_string(operands, operators));
    }
    sort_list(operands, operators);
    println!("{}", "sorted equation:".underline());
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
            2 => abc.0 = get_value(operand.value, index, &operators),
            _ => (),
        };
        abc
    })
}

pub fn resolve(mut operands: Vec<Operand>, mut operators: Vec<Operator>) {
    reduce(&mut operands, &mut operators);
    let is_invalid_expression = operands.iter().any(|operand| {
        (operand.x_power >= 3 || operand.x_power < 0) && operand.value != 0.0
    });
    if is_invalid_expression {
        println!(
                "{}{}",
                "Error".red().bold(),
                ": The expression must be a polynomial expression beetween 0 and 2 degree"
                    .white()
                    .bold(),
            );
        return;
    }
    let degree = operands.iter().map(|operand| {
        operand.x_power
    }).max().unwrap_or(0);
    println!("{}", format!("Polynomial degree: {}", degree).underline());
    let (a, b, c) = find_abc(operands, operators);
    match (a, b, c) {
        (a, b, c) if a != 0.0 => second_degree(a, b, c),
        (_, b, c) if b != 0.0 => first_degree(b, c),
        _ => zero_degree(c),
    }
}
