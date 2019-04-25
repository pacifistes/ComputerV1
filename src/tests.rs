#[cfg(test)]
mod tests {
    use crate::resolver::*;
    use crate::operand::Operand;
    use crate::operator::Operator;

    #[test]
    fn test_hello_world() {
        println!("hello world!");
    }

   #[test]
    fn test_second_degree() {
        second_degree(10.0, 100.0, 1.0);
        second_degree(8.0, 8.0, 2.0);
        second_degree(1.0, 1.0, 1.0);
    }

    #[test]
    fn test_equation_to_string() {
        let operands: Vec<Operand> = vec!(Operand::new(4.0, 0), Operand::new(10.0, 0), Operand::new(25.0, 2), Operand::new(-8.0, 1));
        let operators: Vec<Operator> = vec!(Operator::Add, Operator::Mul, Operator::Sub);
        let expression: String = String::from("4 + 10 * (25 * X^2) - ((-8) * X^1) = 0");
        assert_eq!(expression, equation_to_string(&operands, &operators))
    }
}