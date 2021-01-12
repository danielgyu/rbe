// used to create custom data types

// traditional stcut
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Colors(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn change_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
        
pub fn run() {
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    p.change_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());


    // traditional
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    // tuple
    let mut cs = Colors(255, 0, 0);
    println!("Color: {}, {}, {}", cs.0, cs.1, cs.2);

    cs.0 = 111;
    println!("Color: {}, {}, {}", cs.0, cs.1, cs.2);


}
