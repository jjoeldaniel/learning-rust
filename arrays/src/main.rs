use std::io;

fn main() {

    // array (immutable)
    let arr: [i32;6] = [0, 1, 2, 3, 4, 5];

    loop {

        println!("Please enter an array index.");
        let mut index = String::new();
        
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Invalid index");

        if index >= arr.len() {
            println!("Invalid index");
            continue;
        }

        let element = arr[index];

        println!("The value of the elemnt at index {index} is {element}");

    }
}
