pub fn run() {
    // Default infer
    let x = 1;
    let y = 2.5;

    // Explicit assignment
    let z: i64 = 45454545454;

    // Finding max of a data type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let explicit_active: bool = true;

    println!("{:?}", (x, y, z, is_active));

    // Get boolean from expression
    let is_greater = 10 > 5;
    println!("{is_greater}", is_greater);
}
