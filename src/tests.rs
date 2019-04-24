#[cfg(test)]
mod tests {
    use crate::resolver::*;

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
}