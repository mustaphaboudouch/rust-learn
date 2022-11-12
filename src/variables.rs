pub fn run() {
    // immutable variable
    let age = 24;
    println!("My age is {}", age);

    // mutable variable
    let mut age = 23;
    println!("My age was {}", age);
    age = 24;
    println!("My age now is {}", age);

    // multiple variables
    let (name, age) = ("Mustapha", 24);
    println!("{} is {}", name, age);

    // constants
    const PI: f64 = 3.14;
    println!("PI = {}", PI);
}
