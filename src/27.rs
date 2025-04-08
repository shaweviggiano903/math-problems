fn main() {
    // Example Rust code for solving linear equations.
    let equation: &str = "3x + 5 = 14";
    match equation.split_whitespace().collect::<Vec<&str>>()[0] {
        x if x.is_digit(10) => println!("Solve for x:", x.parse::<f64>().unwrap()),
        _ => println!("Equation not linear."),
    }
}
