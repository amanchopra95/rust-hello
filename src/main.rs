mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;

fn main() {
    println!("================Print==================");
    print::run();
    println!("================Vars==================");
    vars::run();
    println!("================Types==================");
    types::run();
    println!("================Strings==================");
    strings::run();
    println!("================Tuples==================");
    tuples::run();
    println!("================Arrays==================");
    arrays::run();
    println!("================Vectors==================");
    vectors::run();
    println!("================Conditionals==================");
    conditionals::run();
    println!("================Loops==================");
    loops::run();
    println!("================Functions==================");
    functions::run();
    println!("================Pointers==================");
    pointers::run();
    println!("================Structs==================");
    structs::run();


}
