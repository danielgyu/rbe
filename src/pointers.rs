pub fn run() {
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("values: {:?}", (arr1, arr2));

    // non primitive
    // vector
    let vec1 = vec![1, 2, 3];
    // let vec2 = vec1;
    let vec2 = &vec1;
    // println!("values: {:?}", (vec1, vec2));
    println!("values: {:?}", (&vec1, vec2));
    println!("values vec1: {:?}", (vec1));
    println!("values &vec1: {:?}", (&vec1));
    println!("valeus vec2: {:?}", (vec2));
    println!("values &vec2: {:?}", (&vec2));
}
