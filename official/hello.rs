fn main() {
    println!("Hello World");
    println!("I'm a Rustacean!");
    // using comment
    // automatic comment
    /*
     * multi-line comment
     * automatic star
     * with every enter
     */
    let x = 5 + /*in-line comment*/ 5;
    println!("Is 'x' 10 or 100? x= {}", x);
    // formatted print
    // examples of different types
    println!("{0}, this is {1}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             subject="the lazy dog",
             verb="likes",
             object="his master");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("My name is {0}, {1}, {0}", "Bond", "James");
    let pi = 3.141592;
    println!("Pi is roughly {}", pi);
}
