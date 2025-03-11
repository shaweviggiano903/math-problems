fn main() {
    let x = 5;
    let y = 6;
    println!("{} + {} = {}", x, y, add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
