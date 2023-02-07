fn main() {
    
    let mut x: i30 = 0;

    let sum: i32 = loop {
        x += 2;

        if x == 20 {
            break x - 1;
        }

    };

    println!("The sum is {sum}");

    for num in 0..3 {
        println!("{num}");
    }

    for num in (0..3).rev() {
        println!("{num}");
    }

    'loop_a: loop {
        let x = 0;

        if x == 0 {
            println!("Breaking loop_a!");
            break 'loop_a;
        }
    }

    let arr: [i32; 5] = [0, 1, 2, 3, 4];

    for x in arr {
        println!("{}", x);
    }
}
