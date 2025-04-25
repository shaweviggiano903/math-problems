fn main() {
    // Example Rust code for solving mathematical equations
    let equation = String::from("3x + 5 = 14");
    match equation.parse::<i32>() {
        Ok(x) => println!("Solution: x =", x),
        Err(e) => eprintln!("Error: {}", e),
    }
}
