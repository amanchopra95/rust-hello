pub fn run() {
    greetings("Hello", "Jane");
    
    // Let bind function values to a variable

    let sum = add(1,2);
    println!("Sum of 1 and 2 {}", sum);

    // Closures
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3,3));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}