use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single value
    println!("single value: {}", numbers[0]);

    // mutatable
    let mut mut_nums: [i32; 5] = [2,3,4,5,6];
    println!("{:?}", mut_nums);

    mut_nums[2] = 20;
    println!("{:?}", mut_nums);

    // array length
    println!("len: {}", mut_nums.len());

    // stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&mut_nums));

    // get slice
    let slice: &[i32] = &numbers;
    // all
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[0..2];
    println!("Slice from 0 to 2: {:?}", slice);
    println!("Direct slice: {:?}", &numbers[1..3]);

    // length
}
