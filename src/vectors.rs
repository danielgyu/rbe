pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 20;
    println!("{:?}", numbers);
    println!("0: {}, len: {}", numbers[0], numbers.len());

    // add onto vector
    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);

    println!("slice: {:?}", &numbers[1..3]);

}
