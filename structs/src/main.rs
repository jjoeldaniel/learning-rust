struct User {
    active: bool,
    name: String,
    age: u16,    
}

fn main() {

    let user1 = User {
        active: true,
        name: String::from("Joel"),
        age: 22,
    };

    let user2 = User {
        name: String::from("Not Joel"),
        ..user1
    };

    let user3 = build_user(String::from("David"), 35);

    let users = [user1, user2, user3];

    for user in users {
        print_user(&user);
    } 

}

fn print_user(user: &User) {
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Status: {}\n", user.active);
}

fn build_user(name: String, age: u16) -> User {
    User {
        active: true,
        name,
        age,
    }
}
