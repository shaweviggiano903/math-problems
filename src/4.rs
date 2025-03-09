// Generate a random number between 1 and 100
let mut rng = rand::thread_rng();
let num = rng.gen_range(1..=100);

// Print the number to the console
println!("The number is: {}", num);
