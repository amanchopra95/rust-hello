pub fn run() {
    let hello = "Hello";

    let mut hello_string = String::from("Hello");

    println!("Length of hello: {}", hello.len());

    println!("Length of hello string {}", hello_string.len());

    hello_string.push_str(" World");

    println!("{} !", hello_string);
}