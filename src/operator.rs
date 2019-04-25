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
}