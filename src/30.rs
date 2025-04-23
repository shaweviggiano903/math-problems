fn main() {
    // Example code to demonstrate Rust features like generics and lifetimes

    let num1 = 5;
    let num2: i32 = 7;
    let double_num1 = num1 * 2;
    let sum_result = num1 + num2;
    println!("The result of {} + {}", double_num1, sum_result);

    // Example with lifetimes and references
    struct MyStruct {
        _attr: isize,
        reference: &'static str,
    }

    let my_ref = &MyStruct{ ... };

    // Accessing the struct field using the reference
    println!("The value is {}", &my_ref._attr);

    // Accessing the field through a lifetimes
    let ref_val = my_ref.0; // This would compile without error as the lifetime of _attr matches the lifetime of MyStruct

    let mut map: HashMap<String, isize> = HashMap::new();
    match map.insert("key", 1) {
        Ok(_) => println!("Map entry inserted successfully."),
        Err(e) => eprintln!("Map insertion failed: {}", e),
    }
}
