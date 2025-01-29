fn main() {
    // Problem 04
    let mut x = 3; // by default, variable are immutable, so we must add the keyword mut to make them mutable
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
