pub fn run() {
    // simple print
    println!("Hello from print.rs");

    // basic formatting
    println!("My name is {}, i'm {} years old", "Mustapha", 24);

    // expressions
    println!("result = {}", 10 * 4 / 3);

    // positional arguments
    println!("{0} is the best, {0} is the {1}", "Rust", "GOAT");

    // named arguments
    println!(
        "My name is {name}, i'm {age} years old",
        name = "Mustapha",
        age = 24
    );

    // placeholder traits
    println!("binary={:b}, hex={:x}, octal={:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", ("Mustapha", 24, false))
}
