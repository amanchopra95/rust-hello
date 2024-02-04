pub fn run() {
    // Find max of i32
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    println!("Is it active: {}", is_active);

    // Get boolean from expression
    let is_greater = 10 > 5;
    println!("Is 10 > 5 : {}", is_greater);

    // Char
    let a1 = 'a';
    println!("Char in rust: {:?}", a1);
}