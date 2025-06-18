fn main() {
    // {} will be replaced by the argument and the argument is converted to string
    println!("{} is a number", 34); 

    // You can also position the arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // You can also name the arguments
    println!("{subject} {verb} {object}.", object="the lazy dog", verb="jumps over", subject="A quick brown fox");

    // Different formatting can also be invoked by using the format character after :
    println!("69240 in Base 10 is {}", 69240);
    println!("69240 in Base 2 is {:b}", 69240);
    println!("69240 in Base 8 is {:o}", 69240);
    println!("69240 in Base 16 is {:X}", 69240);

    // You can add spaces to justify or pad them with numbers as well
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);

    // You can also used named arguments in the format specifier. Use $ for that.
    println!("{number:>width$}", number=1, width=4);

    // Only types that implement fmt::Display can be formatted with {}. user defined
    // types like struct cant be done.

    // struct Structure(i32);
    // println!("{}", Structure(3)); wont be printed

    // Rust 1.58 and above captures arguments from a surrounding variable as well. 
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("These arguments were captured from surrounding variables: {number:>width$}");


    // we also have fmt::Debug which uses {:?} for debugging purposes

    // Implementing the fmt::Display trait automatically implements 
    // the ToString trait which allows us to convert the type to String.


    let pi = 3.141592;
    println!("Pi with 3 decimal places {pi:.3}");
}