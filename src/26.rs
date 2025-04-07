fn main() {
    let input_str = String::from("Your input here");
    let result = solve(input_str);
    println!("{}", result);
}

fn solve(input: String) -> i32 {
    // Your Rust code for solving the problem goes here
    // For example, if your math-problems (Solutions to math problems) project is about prime numbers:
    // input: 10
    // output: 5
    let factors = input.chars().filter(|&&c| c != '0').map(|&c| (c - '0' as i32, true)).collect::<Vec<_>>();
    let mut sum = 0;
    for &factor in &factors {
        if factor.1 { continue; }
        let divisor = input[(input.len() * 4) + factor.0];
        let quotient = divmod(factor.0, divisor as u32);
        if quotient.0 == 1 && quotient.1 > 5 {
            return 5;
        } else {
            sum += quotient.0;
        }
    }
    sum
}
