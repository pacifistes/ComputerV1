#[cfg(test)]
mod tests {
    use crate::resolver::*;

    #[test]
    fn test_hello_world() {
        println!("hello world!");
    }

   #[test]
    fn test_second_degree() {
        second_degree(1.0, 0.0, 1.0);
        second_degree(0.0, 0.0, 0.0);
        second_degree(0.0, 10.0, 0.0);
    }
}