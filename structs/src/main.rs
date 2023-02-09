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

    print_user(&user1);
    print_user(&user2);
    print_user(&user3);

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
