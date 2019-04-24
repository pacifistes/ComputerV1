pub fn negative_discriminant(a: f64, b: f64, c: f64, d: f64) {
    println!("Discriminanat is strictly negative, the two solutions are: ");
    println!("({} + i * sqrt({})) / {})", -b, -d, a * 2.0);
    println!("({} - i * sqrt({})) / {})", -b, -d, a * 2.0);
}

pub fn null_discriminant(a: f64, b: f64) {
    assert_ne!(a, 0.00);
    println!("Discriminant is null, the solutions is :");
    println!("{}", -b / (2.0 * a));
}

pub fn positive_discriminant(a: f64, b: f64, c: f64, d: f64) {
    assert_ne!(a, 0.00);
    println!("Discriminanat is strictly positive, the two solutions are: ");
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
    println!("The solution is:");
    println!("{}", c / b);
}

pub fn zero_degree() {
    println!("The solutions are all real numbers");
}

