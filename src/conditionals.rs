pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender Says: What would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender Says: Sorry bro, you're not allowed to drink in this bar.");
    } else {
        println!("Bartender Says: I need to check your Id");
    }

    // Shorthand if/else, there are no ternary operators
    let is_of_age = if age >= 21 { true } else { false };
    println!("Great: Is Of Age {}", is_of_age);
}