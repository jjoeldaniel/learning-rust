fn main() {
   
    // Variables are immutable by default
    let mut x: i32 = 5;
    println!("The value of x is: {x}");

    // Mutable variables can be changed
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}",THREE_HOURS_IN_SECONDS);
}
