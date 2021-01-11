pub fn run() {
    let name = "Lee";
    let mut age = 37;

    println!("My name is {} and I am {} years old",
             name, age);

    age = 38;

    println!("My name is {} and I am {} years old",
             name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age) = ("Gyu", 30);
    println!("{} is {}", my_name, my_age);
}
