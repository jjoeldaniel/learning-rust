fn main() {
    
    let mut x: i32 = 0;

    let sum: i32 = loop {
        x += 2;


        if x == 20 {
            break x - 1;
        }

    };

    'loop_a: loop {
        let x = 0;

        if x == 0 {
            println!("Breaking loop_a!");
            break 'loop_a;
        }
    }

    println!("The sum is {sum}");
}
