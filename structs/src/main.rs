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

    let user2 = User {
        name: String::from("Not Joel"),
        ..user1
    };

    let name1: String = user1.name;
    let name2: String = user2.name;

    println!("Name 1: {name1}");
    println!("Name 2: {name2}");

}
