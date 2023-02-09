fn main() {
    
    struct User {
        active: bool,
        name: String,
        age: u16,    
    }
    
    let user1 = User {
        active: true,
        name: String::from("Joel"),
        age: 22,
    };

    let name: String = user1.name;

    println!("{name}");

}
