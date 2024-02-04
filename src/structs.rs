use std::string::String;

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TupleColor(u8,u8,u8);

// Person Struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {   
    // Construct a Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0 
    };

    c.red = 200;

    println!("Colors: {} {} {}", c.red, c.green, c.blue);

    let mut mut_c = TupleColor(255,0,0);
    mut_c.1 = 200;
    println!("Mut Colors: {} {} {}", mut_c.0, mut_c.1, mut_c.2);

    let mut person = Person::new("John", "Dow");
    println!("Person's name is {}", person.full_name());

    person.set_last_name("bro");
    println!("Person LAst name set bro {}", person.last_name);

    println!("Person to  Tuple {:?}", person.to_tuple());
}
