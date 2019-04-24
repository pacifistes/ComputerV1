use regex::Regex;

fn build_regex() -> Regex {
    let float = r"\s*(\-?[0-9]+(?:\.[0-9]+)?)";
    let puissance = r"(?:\s*\*\s*X\s*\^\s*([0-9]+))?";
    let multiple_operand = format!("{}{}{}{}", r"(?:\s*([\+\-\*/])", float, puissance, r")*\s*");
    let expression =  format!("{}{}{}{}{}{}{}{}{}", r"^", float, puissance, multiple_operand, r"(?:(=)", float, puissance, multiple_operand, r")?\s*$");
    Regex::new(&expression[..]).unwrap()
}

pub fn parse(expression: &str) -> Result<i32, &'static str> {
    let regex = build_regex();

    let captures = match regex.captures(expression) {
        Some(captures) => captures,
        None => return Err("Invalid String. The string must be a polynomial expression."),
    };

    // let (operands, operators) = 
    Ok(0)
}

/*
** Vec<Operand>
** Vec<Operator>
*/