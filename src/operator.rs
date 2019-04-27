#[derive(PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn as_str(&self) -> &str {
        match self {
            Operator::Add => " + ",
            Operator::Sub => " - ",
            Operator::Mul => " * ",
            Operator::Div => " / ",
        }
    }

    pub fn new(operator: &str) -> Operator{
        match operator {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            _ => Operator::Div,
        }
    }
}
