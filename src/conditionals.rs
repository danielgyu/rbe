pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_age = true;

    if age >= 21 && check_id || knows_age {
        println!("Bartender: What would like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry");
    } else {
        println!("Bartender: ID please");
    }

    // shorthand if
    let is_of_age = if age > 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
