fn main() {

    let name = String::from("Joel");
    hello_user(name);

    let sum: i32 = add_int(5, 15);
    println!("{}", sum);
}

fn hello_user(name: String) {
    println!("Hello, {name}!");
}

fn add_int(x: i32, y: i32) -> i32 {
    return x + y;
}
