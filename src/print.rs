pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // Basic formatting
    println!("{} am from {}", "I", "Korea");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
             "Lee", "Korea", "code");

    // Named Arguments
    println!("{name} likes to play {activity}",
             name = "Lee",
             activity = "basketball");

    // Placeholder traits
    println!("Binary: {:b}, Hex {:x}, Octal: {:o}",
             10, 10, 10);

    // debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
