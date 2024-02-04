pub fn run() {
    // Print to Console
    println!("Hello from the print.rs file");

    println!("{}", 1);

    println!("{} wow formatting {}", "Kill", "rusty");

    // Positional Arguments
    println!("{0} positional arguments {2} passed in print {1} function ", 0, 1, 2);

    // Named Arguments
    println!("{name} likes to code in {code}", name = "John", code = "rust");

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}