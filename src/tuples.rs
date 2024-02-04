// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 23);

    println!("{} is from {} and he's {} years of age", person.0, person.1, person.2);
}
