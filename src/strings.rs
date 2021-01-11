pub fn run() {
    // primitive str
    let hello = "Hello";

    // string = growable
    let mut hello_grow = String::from("Hello");

    println!("Length: {}", hello.len());
    println!("{}", hello_grow);

    hello_grow.push('P');
    hello_grow.push_str("UsH");

    println!("{}", hello_grow);

    // capacity in bytes
    println!("Capcacity: {}", hello_grow.capacity());

    // check if empty
    println!("Is Empty: {}", hello_grow.is_empty());

    // check contains
    println!("Contains 'Hello' {}", hello_grow.contains("Hello"));

    // replace
    println!("Replace: {}", hello_grow.replace("SH", "sh"));

    // assertion testing
    assert_eq!(2, hello_grow.len());
}
