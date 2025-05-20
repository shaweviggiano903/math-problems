fn main() {
    // Example of creating a simple function in Rust
    let sum = |a: i32, b: i32| -> i32 {
        a + b
    };

    // You can call this function with two arguments and it will return their sum
    println!("The sum is: {}", sum(5, 3));
}
