fn main() {
    let tup: (i32, char, i32) = (123, 'A', 321);

    // Prints 'A'
    println!("{}", tup.1);

    let (x, y, z) = tup;
    
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");
}
