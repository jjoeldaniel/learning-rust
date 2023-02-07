fn main() {
    
    let mut x: i32 = 0;

    let sum: i32 = loop {
        x += 2;


        if x == 20 {
            break x - 1;
        }

    };

    println!("The sum is {sum}");
}
